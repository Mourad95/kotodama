package app.kotomotion.spike

import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp

/** Single screen of the M2 measurement spike: no product UI. */
class MainActivity : ComponentActivity() {
    private val viewModel: SpikeViewModel by viewModels()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                Surface(Modifier.fillMaxSize()) { SpikeScreen(viewModel) }
            }
        }
    }
}

private fun mb(bytes: Long): Long = if (bytes < 0) -1 else bytes / (1024 * 1024)

@Composable
private fun SpikeScreen(vm: SpikeViewModel) {
    val ctx = LocalContext.current
    val debuggable = (ctx.applicationInfo.flags and android.content.pm.ApplicationInfo.FLAG_DEBUGGABLE) != 0
    val ready = vm.engineState is EngineState.Ready
    Column(
        Modifier.safeDrawingPadding().verticalScroll(rememberScrollState()).padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        Text(
            stringResource(
                R.string.device_line,
                "${Build.MANUFACTURER} ${Build.MODEL}", Build.VERSION.RELEASE, Build.VERSION.SDK_INT,
                Build.SUPPORTED_ABIS.firstOrNull() ?: "?", if (debuggable) "debug" else "release",
            ),
            style = MaterialTheme.typography.bodySmall,
        )
        Text(stringResource(R.string.bundle_dir_line, vm.bundleDir.absolutePath), style = MaterialTheme.typography.bodySmall)

        when (val s = vm.engineState) {
            EngineState.Loading -> Text(stringResource(R.string.engine_loading))
            is EngineState.Ready -> {
                Text(stringResource(R.string.cold_start_line, s.coldStartMs))
                Text(stringResource(R.string.open_phases_line, s.verifyMs, s.loadMs), style = MaterialTheme.typography.bodySmall)
                Text(
                    stringResource(
                        R.string.open_memory_line,
                        mb(s.memory.nativeHeap), mb(s.memory.peakRss), mb(s.memory.rss),
                    ),
                    style = MaterialTheme.typography.bodySmall,
                )
            }
            is EngineState.Fallback -> Text(
                if (s.reason == null) stringResource(R.string.bundle_missing)
                else stringResource(R.string.bundle_error, s.reason),
                color = MaterialTheme.colorScheme.error,
            )
        }

        OutlinedTextField(
            value = vm.text,
            onValueChange = { vm.text = it },
            label = { Text(stringResource(R.string.message_label)) },
            modifier = Modifier.fillMaxWidth(),
        )
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            Button(onClick = vm::analyse, enabled = vm.engineState !is EngineState.Loading) {
                Text(stringResource(R.string.analyse_button))
            }
            Button(onClick = vm::runBenchmark, enabled = ready && !vm.benchmarkRunning) {
                Text(stringResource(R.string.benchmark_button))
            }
        }

        vm.analysis?.let { a ->
            val r = a.result
            if (a.errorClass != null) {
                Text(stringResource(R.string.analysis_error, a.errorClass))
            } else if (r == null) {
                Text(stringResource(R.string.result_fallback))
            } else {
                Text(stringResource(R.string.result_effective, r.effective, r.choice, r.confidence))
                Text(stringResource(R.string.latency_line, a.latencyMs))
                r.probabilities.forEach { Text("${it.emotion}: ${"%.3f".format(it.probability)}") }
            }
        }

        if (vm.benchmarkRunning) Text(stringResource(R.string.benchmark_running))
        if (vm.benchmarkMissing) Text(stringResource(R.string.no_bench_messages))
        vm.benchmarkError?.let { Text(stringResource(R.string.analysis_error, it)) }
        vm.benchmark?.let { b ->
            Text(stringResource(R.string.benchmark_result, b.stats.n, b.warmUp, b.stats.p50, b.stats.p95, b.stats.max))
            val m = b.memory
            Text(stringResource(R.string.memory_line, mb(m.nativeHeap), mb(m.javaHeap), mb(m.peakRss), mb(m.rss)))
        }
    }
}
