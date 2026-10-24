package app.kotomotion.spike

import android.app.Application
import android.provider.Settings
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateListOf
import uniffi.kd_ffi.Animator
import uniffi.kd_ffi.MotionMode
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import uniffi.kd_ffi.EmotionEngine
import uniffi.kd_ffi.EmotionResult
import uniffi.kd_ffi.Speller
import uniffi.kd_ffi.SpellingIssue
import uniffi.kd_ffi.intensityLevel
import java.io.File

/** What the engine panel shows. */
sealed interface EngineState {
    data object Loading : EngineState
    data class Ready(val coldStartMs: Long, val verifyMs: Long, val loadMs: Long, val before: MemorySnapshot, val memory: MemorySnapshot) : EngineState
    /** No usable model: every message is treated as "neutre". */
    data class Fallback(val reason: String?) : EngineState
}

/** One analysed message; [result] is null in fallback mode. Holds numbers only, never the text. */
data class AnalysisUi(val result: EmotionResult?, val latencyMs: Double, val errorClass: String? = null)

/**
 * One message of the conversation shown in the app. The conversation lives in memory only and is never
 * stored (docs/legal.md L18). A sent message shows as a sticker that plays when [playToken] changes
 * (ADR-0023).
 */
class ChatMessage(
    val id: Int,
    val text: String,
    val mine: Boolean,
    val emotion: String,
    val writingLevel: Int,
    /** False when the theme shows this emotion as plain text (neutre, ADR-0024). */
    val sticker: Boolean,
) {
    /** Bumped to play the sticker again. */
    var playToken by mutableIntStateOf(0)

    /** The last [playToken] the sticker played: it plays once, then shows its last frame. */
    var playedToken = -1
}

/** The user's intensity setting (docs/roadmap.md): how much motion the animation shows. */
enum class Intensity(val mode: MotionMode) { FULL(MotionMode.FULL), GENTLE(MotionMode.GENTLE), MINIMAL(MotionMode.MINIMAL) }

/** Drawing cost of the last sticker played, in milliseconds per frame (the 60 fps budget is 16.7 ms), and its side in pixels. */
data class RenderStats(val frames: Int, val avgMs: Double, val maxMs: Double, val side: Int)

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

    /** The message being typed; setting it re-checks its spelling after a short pause. */
    var text by mutableStateOf("")
        private set

    fun updateText(value: String) {
        text = value
        scheduleSpellCheck()
    }
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
    private val prefs = app.getSharedPreferences("settings", Application.MODE_PRIVATE)

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
    /**
     * How well the message is written, 1 (several mistakes) to 3 (none). Null: counted from the spelling
     * mistakes of the message (M4); a number: forced by the tester.
     */
    var forcedWritingLevel: Int? by mutableStateOf(null)
        private set

    /** The writing level the next message gets. */
    val writingLevel: Int
        get() = forcedWritingLevel ?: intensityLevel(spellingIssues.size.toUInt()).toInt()

    // --- Spelling (M4, ADR-0025) ------------------------------------------------------------------
    /** Null until the dictionary is loaded, or if it could not be. */
    @Volatile
    private var speller: Speller? = null

    /** Misspelled words of [text] (UTF-16 offsets), from its last check. */
    var spellingIssues: List<SpellingIssue> by mutableStateOf(emptyList())
        private set

    /** The word the suggestions are for, and the suggestions; empty when there is none. */
    var suggestions: Pair<SpellingIssue, List<String>>? by mutableStateOf(null)
        private set
    private var spellJob: Job? = null
    var renderStats: RenderStats? by mutableStateOf(null)
        private set
    /** The conversation, in memory only. */
    val messages = mutableStateListOf(
        ChatMessage(0, "Salut ! Écris-moi un message et regarde-le s'animer.", false, "neutre", 3, false),
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
    fun chooseDarkChat(value: Boolean) {
        darkChat = value
    }
    val effectiveMode: MotionMode get() = if (systemReducedMotion) MotionMode.MINIMAL else intensity.mode

    fun chooseIntensity(value: Intensity) {
        intensity = value
        prefs.edit().putString("intensity", value.name).apply()
    }

    fun chooseEmotion(id: String?) {
        forcedEmotion = id
    }

    fun chooseWritingLevel(value: Int?) {
        forcedWritingLevel = value
    }

    private fun scheduleSpellCheck() {
        spellJob?.cancel()
        val input = text
        if (input.isBlank()) {
            spellingIssues = emptyList()
            suggestions = null
            return
        }
        spellJob = viewModelScope.launch {
            delay(SPELL_DEBOUNCE_MS)
            val s = speller ?: return@launch
            val issues = withContext(Dispatchers.Default) { s.check(input) }
            if (text != input) return@launch
            spellingIssues = issues
            // Suggestions for the first mistake, computed only when it changed (tens of ms).
            val first = issues.firstOrNull()
            val known = suggestions?.takeIf { it.first.word == first?.word }?.second
            val words = when {
                first == null -> null
                known != null -> known
                else -> withContext(Dispatchers.Default) { s.suggest(first.word) }
            }
            if (text == input) suggestions = if (first == null || words == null) null else first to words
        }
    }

    /** Replaces the misspelled word of [issue] by [word]. */
    fun applySuggestion(issue: SpellingIssue, word: String) {
        val start = issue.start.toInt()
        val end = issue.end.toInt()
        if (end > text.length || text.substring(start, end) != issue.word) return
        suggestions = null
        updateText(text.substring(0, start) + word + text.substring(end))
    }

    /** Plays the sticker of a message again (tap on it). */
    fun replay(message: ChatMessage) {
        message.playToken++
    }

    /** Called by a sticker with what its last frames cost to draw. */
    fun reportRender(stats: RenderStats) {
        renderStats = stats
    }

    init {
        viewModelScope.launch(Dispatchers.IO) {
            speller = try {
                fun asset(name: String) = app.assets.open("grammar/$name").bufferedReader().use { it.readText() }
                Speller(asset("fr.aff"), asset("fr.dic"), asset("allowlist.txt"))
            } catch (_: Exception) {
                null // no spelling check: the writing level stays at its maximum
            }
            if (speller != null) withContext(Dispatchers.Main) { scheduleSpellCheck() }
        }
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

    /** Sends the typed message: detects its emotion, then adds it to the conversation as a sticker. */
    fun send() {
        val input = text.trim()
        if (input.isEmpty()) return
        val level = writingLevel
        updateText("")
        viewModelScope.launch {
            val forced = forcedEmotion
            val done = if (forced == null) withContext(engineDispatcher) { analyseOnce(input) } else AnalysisUi(null, 0.0)
            analysis = done
            val emotion = forced ?: done.result?.effective ?: "neutre"
            val app = getApplication<Application>()
            caption = when {
                forced != null -> app.getString(R.string.caption_forced, forced, level)
                done.result != null -> app.getString(R.string.caption_detected, emotion, done.result.confidence, level)
                else -> app.getString(R.string.caption_fallback, level)
            }
            val message = ChatMessage(nextId++, input, true, emotion, level, animator?.isAnimated(emotion) == true)
            messages.add(message)
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
        const val SPELL_DEBOUNCE_MS = 250L
    }
}
