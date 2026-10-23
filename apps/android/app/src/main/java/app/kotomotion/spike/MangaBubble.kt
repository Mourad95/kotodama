package app.kotomotion.spike

import android.graphics.Bitmap
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.produceState
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.FilterQuality
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.StrokeJoin
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.clearAndSetSemantics
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.style.TextGeometricTransform
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlin.math.ceil
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import uniffi.kd_ffi.Animator
import java.nio.ByteBuffer

/**
 * The Rust theme draws the permanent panel and onomatopoeias; the text stays native for wrapping
 * and TalkBack, lettered like a manga effect: bundled font, accent extrusion, halo and slant
 * scaled by the level's depth (ADR-0019).
 */
@Composable
internal fun MangaBubble(vm: SpikeViewModel, message: ChatMessage) {
    val dark = vm.darkChat
    var bounds by remember { mutableStateOf(IntSize.Zero) }
    val illustration by produceState<ImageBitmap?>(null, vm.animator, message.id, dark, bounds) {
        value = null
        val animator: Animator = vm.animator ?: return@produceState
        if (bounds.width <= 0 || bounds.height <= 0) return@produceState
        value = try {
            withContext(Dispatchers.Default) {
                // Full resolution for sharp ink; only huge accessibility layouts are scaled down.
                val ratio = minOf(1f, 2048f / maxOf(bounds.width, bounds.height))
                val width = (bounds.width * ratio).toInt().coerceAtLeast(1)
                val height = (bounds.height * ratio).toInt().coerceAtLeast(1)
                val pixels = animator.renderBubble(message.emotion, message.writingLevel.toUByte(), width.toUInt(), height.toUInt(), dark)
                Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888).apply {
                    copyPixelsFromBuffer(ByteBuffer.wrap(pixels))
                }.asImageBitmap()
            }
        } catch (cancelled: CancellationException) {
            throw cancelled
        } catch (_: Exception) {
            null // Readable native text remains available even if a decoration fails.
        }
    }
    val replayLabel = stringResource(R.string.replay_button)
    Box(
        Modifier.widthIn(min = 120.dp, max = 300.dp)
            .onSizeChanged { bounds = it }
            .clickable(enabled = message.settled, onClickLabel = replayLabel) { vm.replay(message) },
    ) {
        val image = illustration
        if (image == null) {
            Box(Modifier.matchParentSize().background(
                if (dark) Color(0xFF232630) else message.accent.copy(alpha = 0.08f),
                RoundedCornerShape(18.dp),
            ))
        } else {
            Canvas(Modifier.matchParentSize()) {
                drawImage(image, dstSize = IntSize(size.width.toInt(), size.height.toInt()), filterQuality = FilterQuality.High)
            }
        }
        EffectLettering(vm, message, dark)
    }
}

/** Native, accessible message text drawn as manga effect lettering. */
@Composable
private fun EffectLettering(vm: SpikeViewModel, message: ChatMessage, dark: Boolean) {
    val look = remember(vm.animator, message.emotion, message.writingLevel) {
        vm.animator?.let { runCatching { it.bubbleLook(message.emotion, message.writingLevel.toUByte()) }.getOrNull() }
    }
    val assets = LocalContext.current.assets
    val family = remember(assets) {
        runCatching { FontFamily(Font("fonts/NotoSansJP-Bold-subset.ttf", assets)) }.getOrDefault(FontFamily.Default)
    }
    val depth = look?.depth ?: 0f
    val ink = if (dark) Color(0xFFF6F3F4) else look?.ink.toColor() ?: message.accent
    val accent = look?.accent.toColor() ?: message.accent
    val style = TextStyle(
        fontFamily = family,
        fontSize = 18.sp,
        textGeometricTransform = TextGeometricTransform(skewX = -(0.06f + 0.16f * depth)),
    )
    val outline = with(LocalDensity.current) { 3.dp.toPx() }
    Box(Modifier.padding(horizontal = 28.dp, vertical = 30.dp)) {
        // Extrusion: accent copies stepping down-right, each edged in dark ink.
        val steps = ceil(depth * 5).toInt()
        for (i in steps downTo 1) {
            val shift = Modifier.offset(x = (i * 0.6f * 0.65f).dp, y = (i * 0.6f).dp).clearAndSetSemantics {}
            Text(message.text, shift, color = Color(0xFF1B1B22), style = style.copy(drawStyle = Stroke(outline * 0.6f, join = StrokeJoin.Round)))
            Text(message.text, shift, color = accent, style = style)
        }
        Text(
            message.text,
            Modifier.clearAndSetSemantics {},
            color = if (dark) Color(0xFF14151B) else Color.White,
            style = style.copy(drawStyle = Stroke(outline, join = StrokeJoin.Round)),
        )
        Text(message.text, color = ink, style = style)
    }
}

private fun String?.toColor(): Color? =
    this?.let { runCatching { Color(android.graphics.Color.parseColor(it)) }.getOrNull() }
