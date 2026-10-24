package app.kotomotion.spike

import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.viewModels
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.LocalContentColor
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.SegmentedButton
import androidx.compose.material3.SegmentedButtonDefaults
import androidx.compose.material3.SingleChoiceSegmentedButtonRow
import androidx.compose.material3.Surface
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.text.input.OffsetMapping
import androidx.compose.ui.text.input.TransformedText
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.pluralStringResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import uniffi.kd_ffi.SpellingIssue

/**
 * Beta of KotoMotion: a small conversation shown as a chat app would show it. A sent message appears
 * as a sticker that plays its animation once, then stays on its last frame (docs/adr/0023). The
 * conversation is in memory only. Settings and measurements are behind the gear.
 */
class MainActivity : ComponentActivity() {
    private val viewModel: SpikeViewModel by viewModels()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            MaterialTheme {
                Surface(Modifier.fillMaxSize()) { ChatScreen(viewModel) }
            }
        }
    }
}

private fun mb(bytes: Long): Long = if (bytes < 0) -1 else bytes / (1024 * 1024)

@Composable
private fun ChatScreen(vm: SpikeViewModel) {
    var settingsOpen by remember { mutableStateOf(false) }
    val background = if (vm.darkChat) Color(0xFF121212) else Color(0xFFF2F2F5)
    CompositionLocalProvider(LocalContentColor provides if (vm.darkChat) Color.White else Color(0xFF1C1B1F)) {
    Box(Modifier.fillMaxSize().background(background)) {
        Column(Modifier.fillMaxSize().safeDrawingPadding().imePadding()) {
            Row(Modifier.fillMaxWidth().padding(horizontal = 16.dp, vertical = 8.dp), verticalAlignment = Alignment.CenterVertically) {
                Column(Modifier.weight(1f)) {
                    Text(
                        stringResource(R.string.beta_title),
                        style = MaterialTheme.typography.titleMedium,
                        color = if (vm.darkChat) Color.White else Color.Unspecified,
                    )
                    EngineStatus(vm)
                }
                TextButton(onClick = { settingsOpen = !settingsOpen }) {
                    Text(stringResource(if (settingsOpen) R.string.back_to_chat else R.string.settings_button))
                }
            }
            if (settingsOpen) {
                SettingsPanel(vm, Modifier.weight(1f))
            } else {
                MessageList(vm, Modifier.weight(1f).fillMaxWidth())
                if (vm.caption.isNotEmpty()) {
                    Text(
                        vm.caption,
                        Modifier.padding(horizontal = 16.dp),
                        style = MaterialTheme.typography.labelSmall,
                        color = if (vm.darkChat) Color(0xB3FFFFFF) else Color(0x99000000),
                    )
                }
                SpellingBar(vm)
                InputBar(vm)
            }
        }
    }
    }
}

@Composable
private fun EngineStatus(vm: SpikeViewModel) {
    val color = if (vm.darkChat) Color(0xB3FFFFFF) else Color(0x99000000)
    when (val s = vm.engineState) {
        EngineState.Loading -> Text(stringResource(R.string.engine_loading), style = MaterialTheme.typography.labelSmall, color = color)
        is EngineState.Fallback -> Text(
            if (s.reason == null) stringResource(R.string.bundle_missing_short)
            else stringResource(R.string.bundle_error, s.reason),
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.error,
        )
        is EngineState.Ready -> Text(stringResource(R.string.engine_ready), style = MaterialTheme.typography.labelSmall, color = color)
    }
}

@Composable
private fun MessageList(vm: SpikeViewModel, modifier: Modifier) {
    val state = rememberLazyListState()
    LaunchedEffect(vm.messages.size) { state.animateScrollToItem(vm.messages.lastIndex) }
    LazyColumn(
        modifier.fillMaxWidth().padding(horizontal = 12.dp),
        state = state,
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        items(vm.messages, key = { it.id }) { message -> Bubble(vm, message) }
    }
}

@Composable
private fun Bubble(vm: SpikeViewModel, message: ChatMessage) {
    val dark = vm.darkChat
    Row(
        Modifier.fillMaxWidth(),
        horizontalArrangement = if (message.mine) Arrangement.End else Arrangement.Start,
    ) {
        if (message.mine) {
            Column(horizontalAlignment = Alignment.End) {
                if (message.sticker) {
                    StickerCard(vm, message)
                } else {
                    TextBubble(message.text, mine = true, dark = dark)
                }
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text(
                        message.emotion,
                        style = MaterialTheme.typography.labelSmall,
                        color = if (dark) Color(0x99FFFFFF) else Color(0x80000000),
                        modifier = Modifier.padding(horizontal = 6.dp),
                    )
                    ShareButton(vm, message)
                }
            }
        } else {
            TextBubble(message.text, mine = false, dark = dark)
        }
    }
}

/** Sends the message to another app (WhatsApp, …) through the system share sheet, as a GIF sticker. */
@Composable
private fun ShareButton(vm: SpikeViewModel, message: ChatMessage) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()
    var busy by remember { mutableStateOf(false) }
    TextButton(
        enabled = !busy,
        onClick = {
            busy = true
            scope.launch {
                try {
                    StickerShare.share(context, vm.animator, message, vm.effectiveMode)
                } catch (_: Exception) {
                    // Nothing to share with, or the GIF failed: never crash; the text is not in any error.
                } finally {
                    busy = false
                }
            }
        },
    ) { Text(stringResource(if (busy) R.string.share_preparing else R.string.share_button), style = MaterialTheme.typography.labelMedium) }
}

/** Underlines the misspelled words of the input in red (UTF-16 ranges from the speller). */
private class SpellingUnderline(private val issues: List<SpellingIssue>) : VisualTransformation {
    override fun filter(text: AnnotatedString): TransformedText {
        val style = SpanStyle(color = Color(0xFFD32F2F), textDecoration = TextDecoration.Underline)
        val marked = AnnotatedString.Builder(text).apply {
            issues.forEach { issue ->
                val start = issue.start.toInt()
                val end = issue.end.toInt()
                if (end <= text.length && start < end) addStyle(style, start, end)
            }
        }.toAnnotatedString()
        return TransformedText(marked, OffsetMapping.Identity)
    }

    override fun equals(other: Any?): Boolean = other is SpellingUnderline && other.issues == issues
    override fun hashCode(): Int = issues.hashCode()
}

/** The spelling mistakes of the message being typed and the suggestions for the first one. */
@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun SpellingBar(vm: SpikeViewModel) {
    if (vm.text.isBlank()) return
    val count = vm.spellingIssues.size
    val muted = if (vm.darkChat) Color(0xB3FFFFFF) else Color(0x99000000)
    Column(Modifier.fillMaxWidth().padding(horizontal = 16.dp)) {
        Text(
            if (count == 0) stringResource(R.string.spelling_ok, vm.writingLevel)
            else pluralStringResource(R.plurals.spelling_line, count, count, vm.writingLevel),
            style = MaterialTheme.typography.labelSmall,
            color = if (count == 0) muted else MaterialTheme.colorScheme.error,
        )
        vm.suggestions?.let { (issue, words) ->
            FlowRow(horizontalArrangement = Arrangement.spacedBy(6.dp), verticalArrangement = Arrangement.Center) {
                Text(
                    stringResource(R.string.suggestions_for, issue.word),
                    style = MaterialTheme.typography.labelSmall,
                    color = muted,
                    modifier = Modifier.align(Alignment.CenterVertically),
                )
                if (words.isEmpty()) {
                    Text(stringResource(R.string.no_suggestion), style = MaterialTheme.typography.labelSmall, color = muted,
                        modifier = Modifier.align(Alignment.CenterVertically))
                }
                words.forEach { word ->
                    FilterChip(selected = false, onClick = { vm.applySuggestion(issue, word) }, label = { Text(word) })
                }
            }
        }
    }
}

/** A plain chat bubble, for received messages and for emotions the theme does not animate. */
@Composable
private fun TextBubble(text: String, mine: Boolean, dark: Boolean) {
    val fill = when {
        mine && dark -> Color(0xFF1F5C4B)
        mine -> Color(0xFFD9FDD3)
        dark -> Color(0xFF2A2A2E)
        else -> Color.White
    }
    Surface(
        shape = RoundedCornerShape(18.dp),
        color = fill,
        modifier = Modifier.widthIn(max = 300.dp),
    ) {
        Text(
            text,
            Modifier.padding(horizontal = 14.dp, vertical = 10.dp),
            color = if (dark) Color.White else Color(0xFF222222),
        )
    }
}

@Composable
private fun InputBar(vm: SpikeViewModel) {
    Row(
        Modifier.fillMaxWidth().padding(horizontal = 12.dp, vertical = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        OutlinedTextField(
            value = vm.text,
            onValueChange = vm::updateText,
            visualTransformation = SpellingUnderline(vm.spellingIssues),
            placeholder = { Text(stringResource(R.string.message_label)) },
            modifier = Modifier.weight(1f),
            maxLines = 4,
        )
        Button(
            onClick = vm::send,
            enabled = vm.text.isNotBlank() &&
                (vm.engineState !is EngineState.Loading || vm.forcedEmotion != null),
        ) { Text(stringResource(R.string.send_button)) }
    }
}

@Composable
private fun SettingsPanel(vm: SpikeViewModel, modifier: Modifier) {
    val ctx = LocalContext.current
    val debuggable = (ctx.applicationInfo.flags and android.content.pm.ApplicationInfo.FLAG_DEBUGGABLE) != 0
    var showMeasures by remember { mutableStateOf(false) }
    Column(
        modifier.fillMaxWidth().verticalScroll(rememberScrollState()).padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        val textColor = if (vm.darkChat) Color.White else Color.Unspecified
        EmotionChooser(vm)
        AnimationSettings(vm)
        Row(verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(12.dp)) {
            Switch(checked = vm.darkChat, onCheckedChange = vm::chooseDarkChat)
            Text(stringResource(R.string.dark_chat_label))
        }
        TextButton(onClick = { showMeasures = !showMeasures }) {
            Text(stringResource(if (showMeasures) R.string.hide_measures else R.string.show_measures))
        }
        if (showMeasures) MeasuresPanel(vm, debuggable, textColor)
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun EmotionChooser(vm: SpikeViewModel) {
    Text(stringResource(R.string.emotion_label), style = MaterialTheme.typography.titleSmall)
    val emotions = listOf("joie", "colere", "energie", "suspense", "tristesse", "amour", "calme", "genance", "neutre")
    FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        FilterChip(
            selected = vm.forcedEmotion == null,
            onClick = { vm.chooseEmotion(null) },
            label = { Text(stringResource(R.string.emotion_auto)) },
        )
        emotions.forEach { id ->
            FilterChip(selected = vm.forcedEmotion == id, onClick = { vm.chooseEmotion(id) }, label = { Text(stringResource(when (id) {
                "joie" -> R.string.emotion_joy
                "colere" -> R.string.emotion_anger
                "energie" -> R.string.emotion_energy
                "suspense" -> R.string.emotion_suspense
                "tristesse" -> R.string.emotion_sadness
                "amour" -> R.string.emotion_love
                "genance" -> R.string.emotion_embarrassment
                else -> R.string.emotion_calm
            })) })
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
    val levels = listOf(null, 1, 2, 3)
    SingleChoiceSegmentedButtonRow(Modifier.fillMaxWidth()) {
        levels.forEachIndexed { index, level ->
            SegmentedButton(
                selected = vm.forcedWritingLevel == level,
                onClick = { vm.chooseWritingLevel(level) },
                shape = SegmentedButtonDefaults.itemShape(index, levels.size),
            ) { Text(level?.toString() ?: stringResource(R.string.writing_level_auto)) }
        }
    }
}

@Composable
private fun MeasuresPanel(vm: SpikeViewModel, debuggable: Boolean, textColor: Color) {
    val ready = vm.engineState is EngineState.Ready
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        Text(
            stringResource(
                R.string.device_line,
                "${Build.MANUFACTURER} ${Build.MODEL}", Build.VERSION.RELEASE, Build.VERSION.SDK_INT,
                Build.SUPPORTED_ABIS.firstOrNull() ?: "?", if (debuggable) "debug" else "release",
            ),
            style = MaterialTheme.typography.bodySmall,
            color = textColor,
        )
        Text(stringResource(R.string.bundle_dir_line, vm.bundleDir.absolutePath), style = MaterialTheme.typography.bodySmall, color = textColor)
        (vm.engineState as? EngineState.Ready)?.let { s ->
            Text(stringResource(R.string.cold_start_line, s.coldStartMs), color = textColor)
            Text(stringResource(R.string.open_phases_line, s.verifyMs, s.loadMs), style = MaterialTheme.typography.bodySmall, color = textColor)
            Text(
                stringResource(R.string.open_memory_line, mb(s.memory.nativeHeap), mb(s.memory.peakRss), mb(s.memory.rss)),
                style = MaterialTheme.typography.bodySmall,
                color = textColor,
            )
            Text(
                stringResource(R.string.open_split_line, mb(s.before.rss), mb(s.memory.rssAnon), mb(s.memory.rssFile)),
                style = MaterialTheme.typography.bodySmall,
                color = textColor,
            )
        }
        vm.analysis?.let { a ->
            val r = a.result
            if (a.errorClass != null) {
                Text(stringResource(R.string.analysis_error, a.errorClass), color = textColor)
            } else if (r == null) {
                Text(stringResource(R.string.result_fallback), color = textColor)
            } else {
                Text(stringResource(R.string.result_effective, r.effective, r.choice, r.confidence), color = textColor)
                Text(stringResource(R.string.latency_line, a.latencyMs), color = textColor)
                r.probabilities.forEach { Text("${it.emotion}: ${"%.3f".format(it.probability)}", color = textColor) }
            }
        }
        vm.renderStats?.let {
            Text(
                stringResource(R.string.render_stats, it.avgMs, it.maxMs, it.frames, it.side),
                style = MaterialTheme.typography.bodySmall,
                color = textColor,
            )
        }
        OutlinedButton(onClick = vm::runBenchmark, enabled = ready && !vm.benchmarkRunning) {
            Text(stringResource(R.string.benchmark_button))
        }
        if (vm.benchmarkRunning) Text(stringResource(R.string.benchmark_running), color = textColor)
        if (vm.benchmarkMissing) Text(stringResource(R.string.no_bench_messages), color = textColor)
        vm.benchmarkError?.let { Text(stringResource(R.string.analysis_error, it), color = textColor) }
        vm.benchmark?.let { b ->
            Text(stringResource(R.string.benchmark_result, b.stats.n, b.warmUp, b.stats.p50, b.stats.p95, b.stats.max), color = textColor)
            val m = b.memory
            Text(stringResource(R.string.memory_line, mb(m.nativeHeap), mb(m.javaHeap), mb(m.peakRss), mb(m.rss)), color = textColor)
        }
    }
}
