package app.kotomotion.spike

import android.graphics.Bitmap
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
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
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import uniffi.kd_ffi.Animator
import java.nio.ByteBuffer

/** The Rust theme draws the permanent ink; text stays native for wrapping and TalkBack. */
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
                // Bound both sides without changing aspect ratio, including large accessibility fonts.
                val ratio = minOf(1f, 1024f / maxOf(bounds.width, bounds.height))
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
                drawImage(image, dstSize = IntSize(size.width.toInt(), size.height.toInt()), filterQuality = FilterQuality.Medium)
            }
        }
        Text(
            message.text,
            Modifier.padding(horizontal = 28.dp, vertical = 24.dp),
            color = if (dark) Color(0xFFF6F3F4) else message.accent,
        )
    }
}
