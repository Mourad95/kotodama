package app.kotomotion.spike

import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.BackHandler
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.viewModels
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.SegmentedButton
import androidx.compose.material3.SegmentedButtonDefaults
import androidx.compose.material3.SingleChoiceSegmentedButtonRow
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.FilterQuality
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalView
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

/** Beta of KotoMotion: type a message, see its animation on the whole screen. Measurements stay under a toggle. */
class MainActivity : ComponentActivity() {
    private val viewModel: SpikeViewModel by viewModels()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            MaterialTheme {
                Surface(Modifier.fillMaxSize()) {
                    if (viewModel.fullscreen) AnimationScreen(viewModel) else HomeScreen(viewModel)
                }
            }
        }
    }
}

private fun mb(bytes: Long): Long = if (bytes < 0) -1 else bytes / (1024 * 1024)

@Composable
private fun HomeScreen(vm: SpikeViewModel) {
    val ctx = LocalContext.current
    val debuggable = (ctx.applicationInfo.flags and android.content.pm.ApplicationInfo.FLAG_DEBUGGABLE) != 0
    var showMeasures by remember { mutableStateOf(false) }
    Column(
        Modifier.safeDrawingPadding().verticalScroll(rememberScrollState()).padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        Text(stringResource(R.string.beta_title), style = MaterialTheme.typography.headlineSmall)
        OutlinedTextField(
            value = vm.text,
            onValueChange = { vm.text = it },
            label = { Text(stringResource(R.string.message_label)) },
            modifier = Modifier.fillMaxWidth(),
        )
        Button(
            onClick = vm::analyse,
            enabled = vm.text.isNotBlank() && (vm.engineState !is EngineState.Loading || vm.forcedEmotion != null),
            modifier = Modifier.fillMaxWidth(),
        ) { Text(stringResource(R.string.animate_button)) }
        when (val s = vm.engineState) {
            EngineState.Loading -> Text(stringResource(R.string.engine_loading), style = MaterialTheme.typography.bodySmall)
            is EngineState.Fallback -> Text(
                if (s.reason == null) stringResource(R.string.bundle_missing)
                else stringResource(R.string.bundle_error, s.reason),
                color = MaterialTheme.colorScheme.error,
            )
            is EngineState.Ready -> Text(stringResource(R.string.engine_ready), style = MaterialTheme.typography.bodySmall)
        }
        EmotionChooser(vm)
        AnimationSettings(vm)
        TextButton(onClick = { showMeasures = !showMeasures }) {
            Text(stringResource(if (showMeasures) R.string.hide_measures else R.string.show_measures))
        }
        if (showMeasures) MeasuresPanel(vm, debuggable)
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun EmotionChooser(vm: SpikeViewModel) {
    Text(stringResource(R.string.emotion_label), style = MaterialTheme.typography.titleSmall)
    val emotions = listOf("joie", "colere", "energie", "suspense", "tristesse", "amour", "calme")
    FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        FilterChip(
            selected = vm.forcedEmotion == null,
            onClick = { vm.chooseEmotion(null) },
            label = { Text(stringResource(R.string.emotion_auto)) },
        )
        emotions.forEach { id ->
            FilterChip(selected = vm.forcedEmotion == id, onClick = { vm.chooseEmotion(id) }, label = { Text(id) })
        }
    }
}

@Composable
private fun AnimationSettings(vm: SpikeViewModel) {
    if (vm.animator == null) {
        Text(stringResource(R.string.animation_unavailable), color = MaterialTheme.colorScheme.error)
        return
    }
    Text(stringResource(R.string.intensity_label), style = MaterialTheme.typography.titleSmall)
    val labels = listOf(
        Intensity.FULL to R.string.intensity_full,
        Intensity.GENTLE to R.string.intensity_gentle,
        Intensity.MINIMAL to R.string.intensity_minimal,
    )
    SingleChoiceSegmentedButtonRow(Modifier.fillMaxWidth()) {
        labels.forEachIndexed { index, (value, label) ->
            SegmentedButton(
                selected = vm.intensity == value,
                onClick = { vm.chooseIntensity(value) },
                shape = SegmentedButtonDefaults.itemShape(index, labels.size),
                enabled = !vm.systemReducedMotion,
            ) { Text(stringResource(label)) }
        }
    }
    if (vm.systemReducedMotion) {
        Text(stringResource(R.string.reduced_motion_forced), style = MaterialTheme.typography.bodySmall)
    }
    Text(stringResource(R.string.writing_level_label, vm.writingLevel), style = MaterialTheme.typography.titleSmall)
    SingleChoiceSegmentedButtonRow(Modifier.fillMaxWidth()) {
        (1..3).forEach { level ->
            SegmentedButton(
                selected = vm.writingLevel == level,
                onClick = { vm.chooseWritingLevel(level) },
                shape = SegmentedButtonDefaults.itemShape(level - 1, 3),
            ) { Text(level.toString()) }
        }
    }
}

/** The animation on the whole screen. Tap to replay, back to leave. */
@Composable
private fun AnimationScreen(vm: SpikeViewModel) {
    BackHandler { vm.closeAnimation() }
    val view = LocalView.current
    DisposableEffect(view) {
        val window = (view.context as ComponentActivity).window
        val controller = WindowCompat.getInsetsController(window, view)
        controller.systemBarsBehavior = WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
        controller.hide(WindowInsetsCompat.Type.systemBars())
        onDispose { controller.show(WindowInsetsCompat.Type.systemBars()) }
    }
    Box(Modifier.fillMaxSize().clickable { vm.replay() }) {
        // Reading frameTick in the draw scope redraws the canvas for every new frame.
        Canvas(Modifier.fillMaxSize()) {
            vm.frameTick
            if (vm.hasPlayed) {
                drawImage(
                    vm.frame,
                    dstSize = IntSize(size.width.toInt(), size.height.toInt()),
                    filterQuality = FilterQuality.Medium,
                )
            }
        }
        Row(
            Modifier.align(Alignment.TopEnd).safeDrawingPadding().padding(8.dp),
            horizontalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            TextButton(onClick = { vm.closeAnimation() }) {
                Text(stringResource(R.string.close_button), color = Color(0x99000000))
            }
        }
        Text(
            vm.caption,
            Modifier.align(Alignment.BottomCenter).safeDrawingPadding().padding(12.dp),
            style = MaterialTheme.typography.labelSmall,
            color = Color(0x80000000),
        )
    }
}

@Composable
private fun MeasuresPanel(vm: SpikeViewModel, debuggable: Boolean) {
    val ready = vm.engineState is EngineState.Ready
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        Text(
            stringResource(
                R.string.device_line,
                "${Build.MANUFACTURER} ${Build.MODEL}", Build.VERSION.RELEASE, Build.VERSION.SDK_INT,
                Build.SUPPORTED_ABIS.firstOrNull() ?: "?", if (debuggable) "debug" else "release",
            ),
            style = MaterialTheme.typography.bodySmall,
        )
        Text(stringResource(R.string.bundle_dir_line, vm.bundleDir.absolutePath), style = MaterialTheme.typography.bodySmall)
        (vm.engineState as? EngineState.Ready)?.let { s ->
            Text(stringResource(R.string.cold_start_line, s.coldStartMs))
            Text(stringResource(R.string.open_phases_line, s.verifyMs, s.loadMs), style = MaterialTheme.typography.bodySmall)
            Text(
                stringResource(R.string.open_memory_line, mb(s.memory.nativeHeap), mb(s.memory.peakRss), mb(s.memory.rss)),
                style = MaterialTheme.typography.bodySmall,
            )
            Text(
                stringResource(R.string.open_split_line, mb(s.before.rss), mb(s.memory.rssAnon), mb(s.memory.rssFile)),
                style = MaterialTheme.typography.bodySmall,
            )
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
        vm.renderStats?.let {
            Text(
                stringResource(R.string.render_stats, it.avgMs, it.maxMs, it.frames, it.width),
                style = MaterialTheme.typography.bodySmall,
            )
        }
        OutlinedButton(onClick = vm::runBenchmark, enabled = ready && !vm.benchmarkRunning) {
            Text(stringResource(R.string.benchmark_button))
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
