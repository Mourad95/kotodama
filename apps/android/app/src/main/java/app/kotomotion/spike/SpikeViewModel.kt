package app.kotomotion.spike

import android.app.Application
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
    data class Ready(val coldStartMs: Long) : EngineState
    /** No usable model: every message is treated as "calme". */
    data class Fallback(val reason: String?) : EngineState
}

/** One analysed message; [result] is null in fallback mode. Holds numbers only, never the text. */
data class AnalysisUi(val result: EmotionResult?, val latencyMs: Double)

/** A finished benchmark run. */
data class BenchmarkUi(val stats: LatencyStats, val warmUp: Int, val memory: MemorySnapshot)

/**
 * Holds the engine and the screen state. The typed text lives in memory only (never logged,
 * never persisted: docs/legal.md L18). All engine calls run on a single background thread.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class SpikeViewModel(app: Application) : AndroidViewModel(app) {
    private val engineDispatcher: CoroutineDispatcher = Dispatchers.Default.limitedParallelism(1)
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

    init {
        val ortPath = app.applicationInfo.nativeLibraryDir + "/libonnxruntime.so"
        viewModelScope.launch(engineDispatcher) {
            val state = if (!File(bundleDir, "manifest.json").isFile) {
                EngineState.Fallback(null)
            } else {
                val t0 = System.nanoTime()
                try {
                    engine = EmotionEngine.open(bundleDir.absolutePath, ortPath)
                    EngineState.Ready((System.nanoTime() - t0) / 1_000_000)
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
            analysis = withContext(engineDispatcher) { analyseOnce(input) }
        }
    }

    private fun analyseOnce(input: String): AnalysisUi {
        val e = engine ?: return AnalysisUi(null, 0.0)
        val t0 = System.nanoTime()
        val result = e.analyze(input)
        return AnalysisUi(result, (System.nanoTime() - t0) / 1e6)
    }

    fun runBenchmark() {
        val e = engine ?: return
        if (benchmarkRunning) return
        benchmarkRunning = true
        benchmarkMissing = false
        viewModelScope.launch {
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
            benchmarkRunning = false
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
    }
}
