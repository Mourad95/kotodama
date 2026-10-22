package app.kotomotion.spike

import android.os.Debug
import java.io.File

/** Latency summary of a benchmark run, in milliseconds. */
data class LatencyStats(val n: Int, val p50: Double, val p95: Double, val max: Double)

/** Memory snapshot in bytes (`-1` when unavailable). */
data class MemorySnapshot(val nativeHeap: Long, val javaHeap: Long, val peakRss: Long, val rss: Long)

/** Nearest-rank percentile over [sortedMs] (ascending, non-empty). */
fun percentile(sortedMs: List<Double>, p: Double): Double {
    val rank = Math.ceil(p / 100.0 * sortedMs.size).toInt().coerceIn(1, sortedMs.size)
    return sortedMs[rank - 1]
}

/** Summarises per-call latencies (milliseconds). */
fun summarize(latenciesMs: List<Double>): LatencyStats {
    val sorted = latenciesMs.sorted()
    return LatencyStats(sorted.size, percentile(sorted, 50.0), percentile(sorted, 95.0), sorted.last())
}

/** Reads native heap, Java heap and `/proc/self/status` VmHWM (peak RSS) / VmRSS. */
fun memorySnapshot(): MemorySnapshot {
    val rt = Runtime.getRuntime()
    val status = try {
        File("/proc/self/status").readLines()
    } catch (_: Exception) {
        emptyList()
    }
    fun kb(key: String): Long =
        status.firstOrNull { it.startsWith("$key:") }
            ?.filter { it.isDigit() }?.toLongOrNull()?.times(1024) ?: -1L
    return MemorySnapshot(
        nativeHeap = Debug.getNativeHeapAllocatedSize(),
        javaHeap = rt.totalMemory() - rt.freeMemory(),
        peakRss = kb("VmHWM"),
        rss = kb("VmRSS"),
    )
}
