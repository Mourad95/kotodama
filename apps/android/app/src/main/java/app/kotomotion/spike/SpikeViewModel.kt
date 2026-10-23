package app.kotomotion.spike

import android.app.Application
import android.graphics.Bitmap
import android.os.SystemClock
import android.provider.Settings
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.isActive
import uniffi.kd_ffi.Animator
import uniffi.kd_ffi.FrameLayer
import kotlinx.coroutines.async
import kotlinx.coroutines.coroutineScope
import uniffi.kd_ffi.MotionMode
import java.nio.ByteBuffer
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.kd_ffi.EmotionEngine
import uniffi.kd_ffi.EmotionResult
import java.io.File

/** What the engine panel shows. */
sealed interface EngineState {
    data object Loading : EngineState
    data class Ready(val coldStartMs: Long, val verifyMs: Long, val loadMs: Long, val before: MemorySnapshot, val memory: MemorySnapshot) : EngineState
    /** No usable model: every message is treated as "calme". */
    data class Fallback(val reason: String?) : EngineState
}

/** One analysed message; [result] is null in fallback mode. Holds numbers only, never the text. */
data class AnalysisUi(val result: EmotionResult?, val latencyMs: Double, val errorClass: String? = null)

/**
 * One message of the conversation shown in the app. The conversation lives in memory only and is never
 * stored (docs/legal.md L18). [settled] turns true when its bubble shows: as soon as its animation starts,
 * since the animation itself no longer draws the message (ADR-0019).
 */
class ChatMessage(
    val id: Int,
    val text: String,
    val mine: Boolean,
    val emotion: String,
    val accent: Color,
    val writingLevel: Int,
    settled: Boolean,
) {
    var settled by mutableStateOf(settled)
}

/** The user's intensity setting (docs/roadmap.md): how much motion the animation shows. */
enum class Intensity(val mode: MotionMode) { FULL(MotionMode.FULL), GENTLE(MotionMode.GENTLE), MINIMAL(MotionMode.MINIMAL) }

/**
 * Drawing cost of the animation, in milliseconds per frame (the 60 fps budget is 16.7 ms), and the widths
 * in pixels the two layers were drawn at last (ADR-0017).
 */
data class RenderStats(val frames: Int, val avgMs: Double, val maxMs: Double, val frontWidth: Int, val backWidth: Int)

/** A finished benchmark run. */
data class BenchmarkUi(val stats: LatencyStats, val warmUp: Int, val memory: MemorySnapshot)

/**
 * Holds the engine and the screen state. The typed text lives in memory only (never logged,
 * never persisted: docs/legal.md L18). All engine calls run on a single background thread.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class SpikeViewModel(app: Application) : AndroidViewModel(app) {
    private val engineDispatcher: CoroutineDispatcher = Dispatchers.Default.limitedParallelism(1)
    @Volatile
    private var engine: EmotionEngine? = null

    var bundleDir: File by mutableStateOf(File(app.filesDir, "models"))
        private set

    var text by mutableStateOf("")
    var engineState: EngineState by mutableStateOf(EngineState.Loading)
        private set
    var analysis: AnalysisUi? by mutableStateOf(null)
        private set
    var benchmark: BenchmarkUi? by mutableStateOf(null)
        private set
    var benchmarkRunning by mutableStateOf(false)
        private set
    var benchmarkMissing by mutableStateOf(false)
        private set
    var benchmarkError: String? by mutableStateOf(null)
        private set

    // --- Animation (M3) -----------------------------------------------------------------------
    // One thread per layer: the two layers of a frame are drawn at the same time.
    private val backDispatcher: CoroutineDispatcher = Dispatchers.Default.limitedParallelism(1)
    private val frontDispatcher: CoroutineDispatcher = Dispatchers.Default.limitedParallelism(1)
    private val prefs = app.getSharedPreferences("settings", Application.MODE_PRIVATE)
    private var animJob: Job? = null

    /** Null if the theme could not be loaded: the screen then shows no animation. */
    val animator: Animator? = try {
        Animator(app.assets.open("theme.json").bufferedReader().use { it.readText() })
    } catch (_: Exception) {
        null
    }

    /** The system asked for no animations (animator duration scale 0): the minimal mode is forced. */
    val systemReducedMotion: Boolean =
        Settings.Global.getFloat(app.contentResolver, Settings.Global.ANIMATOR_DURATION_SCALE, 1f) == 0f

    var intensity by mutableStateOf(
        Intensity.entries.firstOrNull { it.name == prefs.getString("intensity", null) } ?: Intensity.FULL,
    )
        private set
    /** How well the message is written, 1 (several mistakes) to 3 (none); chosen by hand in this spike. */
    var writingLevel by mutableIntStateOf(3)
        private set
    var frameTick by mutableIntStateOf(0)
        private set
    var renderStats: RenderStats? by mutableStateOf(null)
        private set
    private var backBitmap: Bitmap = Bitmap.createBitmap(1, 1, Bitmap.Config.ARGB_8888)
    private var frontBitmap: Bitmap = Bitmap.createBitmap(1, 1, Bitmap.Config.ARGB_8888)
    /** The soft screen effects of the frame, drawn small: stretch them over the animation area. */
    var backFrame: ImageBitmap by mutableStateOf(backBitmap.asImageBitmap())
        private set
    /** The particles, message and onomatopoeias, drawn at (nearly) screen resolution, over [backFrame]. */
    var frontFrame: ImageBitmap by mutableStateOf(frontBitmap.asImageBitmap())
        private set
    /** True while the animation plays over the conversation. */
    var overlayActive by mutableStateOf(false)
        private set
    /** Opacity of the overlay: 1 while playing, fading to 0 when the message returns to its bubble. */
    var overlayAlpha by mutableFloatStateOf(1f)
        private set
    /** The conversation, in memory only. */
    val messages = mutableStateListOf(
        ChatMessage(0, "Salut ! Écris-moi un message et regarde-le s'animer.", false, "calme", Color(0xFF085041), 3, true),
    )
    private var nextId = 1
    /** Dark conversation background, to check the message stays readable over it. */
    var darkChat by mutableStateOf(false)
        private set
    /** Emotion forced by the tester (null = the detected one), to see every animation without the model. */
    var forcedEmotion: String? by mutableStateOf(null)
        private set
    /** Short description of the last animation ("joie, confiance 0.71"), for the testers. */
    var caption by mutableStateOf("")
        private set
    private var viewportWidth: Int = app.resources.displayMetrics.widthPixels
    private var viewportHeight: Int = app.resources.displayMetrics.heightPixels

    /** Called with the size in pixels of the area the animation plays over, so frames match it exactly. */
    fun setViewport(widthPx: Int, heightPx: Int) {
        if (widthPx > 0 && heightPx > 0) {
            viewportWidth = widthPx
            viewportHeight = heightPx
        }
    }

    fun chooseDarkChat(value: Boolean) {
        darkChat = value
    }
    var hasPlayed by mutableStateOf(false)
        private set

    val effectiveMode: MotionMode get() = if (systemReducedMotion) MotionMode.MINIMAL else intensity.mode

    fun chooseIntensity(value: Intensity) {
        intensity = value
        prefs.edit().putString("intensity", value.name).apply()
    }

    fun chooseEmotion(id: String?) {
        forcedEmotion = id
    }

    /** Ends the animation now: the message goes straight back to its bubble. */
    fun skipAnimation() {
        animJob?.cancel()
    }

    fun chooseWritingLevel(value: Int) {
        writingLevel = value
    }

    /** Plays the animation of a message again (tap on its bubble); the bubble stays visible. */
    fun replay(message: ChatMessage) {
        if (!overlayActive) play(message)
    }

    /**
     * Plays the animation of [message] over the conversation, around its bubble, then fades the overlay
     * out. The text is only handed to the renderer: it is never logged or stored, and the overlay does not
     * draw it (ADR-0019). Each frame is drawn as two layers at once (ADR-0017): the soft effects small, the
     * inked lines, particles and onomatopoeias at screen resolution. While frames take too long, only the
     * soft layer is drawn smaller, one quality step at a time: the inked layer never loses sharpness.
     */
    private fun play(message: ChatMessage) {
        val animator = animator ?: run { message.settled = true; return }
        animJob?.cancel()
        animJob = viewModelScope.launch {
            overlayActive = true
            overlayAlpha = 1f
            // The overlay plays the effects around the bubble, which carries the message from the start.
            message.settled = true
            try {
                val emotion = message.emotion
                val text = message.text
                val mode = effectiveMode
                val level = message.writingLevel.toUByte()
                val duration = animator.durationMs(emotion, level, mode, text.length.toUInt()).toLong()
                var step = 0
                val areaWidth = viewportWidth
                val aspect = viewportHeight.toFloat() / areaWidth
                fun width(scale: Float) = (areaWidth * scale).toInt().coerceIn(1, MAX_SIDE)
                fun height(width: Int) = (width * aspect).toInt().coerceIn(1, MAX_SIDE)
                var frontWidth = 0
                var backWidth = 0
                fun allocate() {
                    frontWidth = width(QUALITY_STEPS[step].first)
                    backWidth = width(QUALITY_STEPS[step].second)
                    frontBitmap = Bitmap.createBitmap(frontWidth, height(frontWidth), Bitmap.Config.ARGB_8888)
                    backBitmap = Bitmap.createBitmap(backWidth, height(backWidth), Bitmap.Config.ARGB_8888)
                    frontFrame = frontBitmap.asImageBitmap()
                    backFrame = backBitmap.asImageBitmap()
                }
                allocate()
                val start = SystemClock.elapsedRealtime()
                var frames = 0
                var total = 0.0
                var worst = 0.0
                var recent = 0.0
                while (isActive) {
                    val t = (SystemClock.elapsedRealtime() - start).coerceAtMost(duration)
                    val begun = System.nanoTime()
                    fun draw(layer: FrameLayer, w: Int) =
                        animator.render(emotion, level, mode, text, t.toUInt(), w.toUInt(), height(w).toUInt(), SEED, layer)
                    val (back, front) = try {
                        coroutineScope {
                            val back = async(backDispatcher) { draw(FrameLayer.BACK, backWidth) }
                            val front = withContext(frontDispatcher) { draw(FrameLayer.FRONT, frontWidth) }
                            back.await() to front
                        }
                    } catch (_: Exception) {
                        break // never crash the app for a failed frame; the text is not part of any error
                    }
                    backBitmap.copyPixelsFromBuffer(ByteBuffer.wrap(back))
                    frontBitmap.copyPixelsFromBuffer(ByteBuffer.wrap(front))
                    val spent = (System.nanoTime() - begun) / 1e6
                    frames++; total += spent; worst = maxOf(worst, spent); recent += spent
                    frameTick++
                    hasPlayed = true
                    renderStats = RenderStats(frames, total / frames, worst, frontWidth, backWidth)
                    if (frames % ADAPT_EVERY == 0) {
                        // Too slow: draw the soft layer smaller (it is stretched to the animation area).
                        if (recent / ADAPT_EVERY > SLOW_FRAME_MS && step < QUALITY_STEPS.lastIndex) {
                            step++
                            allocate()
                        }
                        recent = 0.0
                    }
                    if (t >= duration) break
                    if (spent < FRAME_INTERVAL_MS) delay((FRAME_INTERVAL_MS - spent).toLong())
                }
                // The animation is over: fade the overlay out while the message returns to its bubble.
                val fadeStart = SystemClock.elapsedRealtime()
                while (isActive) {
                    val left = 1f - (SystemClock.elapsedRealtime() - fadeStart) / FADE_OUT_MS
                    overlayAlpha = left.coerceIn(0f, 1f)
                    frameTick++
                    if (left <= 0f) break
                    delay(FRAME_INTERVAL_MS.toLong())
                }
            } finally {
                // Also when skipped or cancelled: never leave the overlay on screen.
                overlayActive = false
                message.settled = true
            }
        }
    }

    init {
        val ortPath = app.applicationInfo.nativeLibraryDir + "/libonnxruntime.so"
        // Private storage: only the app can write the receipt that lets later opens skip the checksums.
        val receipt = File(app.filesDir, "bundle.receipt")
        viewModelScope.launch(engineDispatcher) {
            val before = memorySnapshot()
            val t0 = System.nanoTime()
            val state = try {
                val prepared = BundledModel.prepare(File(app.filesDir, "models")) { path -> app.assets.open(path) }
                bundleDir = prepared
                val opened = EmotionEngine.open(prepared.absolutePath, ortPath, receipt.absolutePath)
                engine = opened
                val timings = opened.openTimings()
                EngineState.Ready(
                    (System.nanoTime() - t0) / 1_000_000,
                    timings.verifyMs.toLong(), timings.loadMs.toLong(), before, memorySnapshot(),
                )
            } catch (e: Exception) {
                EngineState.Fallback(e.javaClass.simpleName)
            } catch (e: UnsatisfiedLinkError) {
                EngineState.Fallback(e.javaClass.simpleName)
            }
            engineState = state
        }
    }

    /** Sends the typed message: detects its emotion, then plays its animation over the conversation. */
    fun send() {
        val input = text.trim()
        if (input.isEmpty() || overlayActive) return
        text = ""
        viewModelScope.launch {
            val forced = forcedEmotion
            val done = if (forced == null) withContext(engineDispatcher) { analyseOnce(input) } else AnalysisUi(null, 0.0)
            analysis = done
            val emotion = forced ?: done.result?.effective ?: "calme"
            val app = getApplication<Application>()
            caption = when {
                forced != null -> app.getString(R.string.caption_forced, forced, writingLevel)
                done.result != null -> app.getString(R.string.caption_detected, emotion, done.result.confidence, writingLevel)
                else -> app.getString(R.string.caption_fallback, writingLevel)
            }
            val accent = animator?.accentColor(emotion)?.let { runCatching { Color(android.graphics.Color.parseColor(it)) }.getOrNull() }
                ?: Color(0xFF444444)
            val message = ChatMessage(nextId++, input, true, emotion, accent, writingLevel, false)
            messages.add(message)
            play(message)
        }
    }

    private fun analyseOnce(input: String): AnalysisUi {
        val e = engine ?: return AnalysisUi(null, 0.0)
        val t0 = System.nanoTime()
        return try {
            val result = e.analyze(input)
            AnalysisUi(result, (System.nanoTime() - t0) / 1e6)
        } catch (ex: Exception) {
            // Only the exception class (EngineException.*): never its text.
            AnalysisUi(null, 0.0, ex.javaClass.simpleName)
        }
    }

    fun runBenchmark() {
        val e = engine ?: return
        if (benchmarkRunning) return
        benchmarkRunning = true
        benchmarkMissing = false
        benchmarkError = null
        viewModelScope.launch {
            try {
                val run = withContext(engineDispatcher) {
                    val messages = loadBenchMessages()
                    if (messages.isEmpty()) return@withContext null
                    val warmUp = minOf(WARM_UP, messages.size)
                    repeat(warmUp) { e.analyze(messages[it]) }
                    val latencies = messages.map { m ->
                        val t0 = System.nanoTime()
                        e.analyze(m)
                        (System.nanoTime() - t0) / 1e6
                    }
                    BenchmarkUi(summarize(latencies), warmUp, memorySnapshot())
                }
                benchmark = run
                benchmarkMissing = run == null
            } catch (ex: Exception) {
                // Only the exception class: never its text.
                benchmarkError = ex.javaClass.simpleName
            } finally {
                benchmarkRunning = false
            }
        }
    }

    private fun loadBenchMessages(): List<String> =
        getApplication<Application>().assets.open("bench_messages.txt").bufferedReader().useLines { lines ->
            lines.map { it.trim() }.filter { it.isNotEmpty() }.toList()
        }

    override fun onCleared() {
        engine?.close()
        engine = null
    }

    private companion object {
        const val WARM_UP = 5
        /**
         * Drawing sizes of the front and back layers as fractions of the animation area's width, from the
         * sharpest to the cheapest (ADR-0017). The front layer holds every hard edge, so it always stays at
         * screen resolution (ADR-0019); only the soft back layer shrinks.
         */
        val QUALITY_STEPS = listOf(1f to 0.5f, 1f to 0.4f, 1f to 0.33f)
        /** Largest frame side the engine draws. */
        const val MAX_SIDE = 2048
        const val ADAPT_EVERY = 12
        const val SLOW_FRAME_MS = 28.0
        const val SEED = 1UL
        const val FRAME_INTERVAL_MS = 16.0
        const val FADE_OUT_MS = 350f
    }
}
