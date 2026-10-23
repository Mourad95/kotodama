package app.kotomotion.spike

import android.app.Application
import android.graphics.Bitmap
import android.os.SystemClock
import android.provider.Settings
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.isActive
import uniffi.kd_ffi.Animator
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

/** The user's intensity setting (docs/roadmap.md): how much motion the animation shows. */
enum class Intensity(val mode: MotionMode) { FULL(MotionMode.FULL), GENTLE(MotionMode.GENTLE), MINIMAL(MotionMode.MINIMAL) }

/** Drawing cost of the animation, in milliseconds per frame (the 60 fps budget is 16.7 ms). */
data class RenderStats(val frames: Int, val avgMs: Double, val maxMs: Double)

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

    val bundleDir: File = File(app.getExternalFilesDir(null) ?: app.filesDir, "bundle")

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
    private val animDispatcher: CoroutineDispatcher = Dispatchers.Default.limitedParallelism(1)
    private val prefs = app.getSharedPreferences("settings", Application.MODE_PRIVATE)
    private var animJob: Job? = null
    private var lastPlayed: Pair<String, String>? = null

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
    val frame: ImageBitmap get() = frameBitmap.asImageBitmap()
    private val frameBitmap: Bitmap = Bitmap.createBitmap(FRAME_W, FRAME_H, Bitmap.Config.ARGB_8888)
    var hasPlayed by mutableStateOf(false)
        private set

    val effectiveMode: MotionMode get() = if (systemReducedMotion) MotionMode.MINIMAL else intensity.mode

    fun chooseIntensity(value: Intensity) {
        intensity = value
        prefs.edit().putString("intensity", value.name).apply()
        replay()
    }

    fun chooseWritingLevel(value: Int) {
        writingLevel = value
        replay()
    }

    fun replay() {
        lastPlayed?.let { (emotion, message) -> play(emotion, message) }
    }

    /**
     * Plays the animation of [message] for [emotion], then keeps the last frame (the settled message).
     * The text is only handed to the renderer: it is never logged or stored.
     */
    private fun play(emotion: String, message: String) {
        val animator = animator ?: return
        lastPlayed = emotion to message
        animJob?.cancel()
        animJob = viewModelScope.launch {
            val mode = effectiveMode
            val level = writingLevel.toUByte()
            val duration = animator.durationMs(emotion, level, mode, message.length.toUInt()).toLong()
            val start = SystemClock.elapsedRealtime()
            var frames = 0
            var total = 0.0
            var worst = 0.0
            while (isActive) {
                val t = (SystemClock.elapsedRealtime() - start).coerceAtMost(duration)
                val begun = System.nanoTime()
                val pixels = try {
                    withContext(animDispatcher) {
                        animator.render(emotion, level, mode, message, t.toUInt(), FRAME_W.toUInt(), FRAME_H.toUInt(), SEED)
                    }
                } catch (_: Exception) {
                    break // never crash the app for a failed frame; the text is not part of any error
                }
                frameBitmap.copyPixelsFromBuffer(ByteBuffer.wrap(pixels))
                val spent = (System.nanoTime() - begun) / 1e6
                frames++; total += spent; worst = maxOf(worst, spent)
                frameTick++
                hasPlayed = true
                renderStats = RenderStats(frames, total / frames, worst)
                if (t >= duration) break // the last frame stays on screen
                if (spent < FRAME_INTERVAL_MS) delay((FRAME_INTERVAL_MS - spent).toLong())
            }
        }
    }

    init {
        val ortPath = app.applicationInfo.nativeLibraryDir + "/libonnxruntime.so"
        // Private storage: only the app can write the receipt that lets later opens skip the checksums.
        val receipt = File(app.filesDir, "bundle.receipt")
        viewModelScope.launch(engineDispatcher) {
            val state = if (!File(bundleDir, "manifest.json").isFile) {
                EngineState.Fallback(null)
            } else {
                val before = memorySnapshot()
                val t0 = System.nanoTime()
                try {
                    val opened = EmotionEngine.open(bundleDir.absolutePath, ortPath, receipt.absolutePath)
                    engine = opened
                    val timings = opened.openTimings()
                    EngineState.Ready(
                        (System.nanoTime() - t0) / 1_000_000,
                        timings.verifyMs.toLong(), timings.loadMs.toLong(), before, memorySnapshot(),
                    )
                } catch (e: Exception) {
                    // Only the exception class: its message could in theory echo input.
                    EngineState.Fallback(e.javaClass.simpleName)
                } catch (e: UnsatisfiedLinkError) {
                    EngineState.Fallback(e.javaClass.simpleName)
                }
            }
            engineState = state
        }
    }

    fun analyse() {
        val input = text
        viewModelScope.launch {
            val done = withContext(engineDispatcher) { analyseOnce(input) }
            analysis = done
            play(done.result?.effective ?: "calme", input)
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
        const val FRAME_W = 540
        const val FRAME_H = 960
        const val SEED = 1UL
        const val FRAME_INTERVAL_MS = 16.0
    }
}
