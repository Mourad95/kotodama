package app.kotomotion.spike

import android.graphics.Bitmap
import android.os.SystemClock
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.FilterQuality
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.semantics.contentDescription
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.withContext
import uniffi.kd_ffi.FrameLayer
import uniffi.kd_ffi.MotionMode
import java.nio.ByteBuffer

/**
 * A sent message shown as a sticker, as a chat app shows one (ADR-0023): a square card that plays the
 * animation once, then stays on its last frame, the whole scene with the message. Tap to play it again.
 * TalkBack reads the message text.
 */
@Composable
internal fun StickerCard(vm: SpikeViewModel, message: ChatMessage) {
    val side = with(LocalDensity.current) { STICKER_SIZE.roundToPx() }.coerceIn(1, MAX_STICKER_SIDE)
    val bitmap = remember(side) { Bitmap.createBitmap(side, side, Bitmap.Config.ARGB_8888) }
    val image = remember(bitmap) { bitmap.asImageBitmap() }
    var tick by remember { mutableIntStateOf(0) }
    val mode = vm.effectiveMode
    val token = message.playToken
    LaunchedEffect(vm.animator, side, mode, token) {
        val animator = vm.animator ?: return@LaunchedEffect
        val emotion = message.emotion
        val level = message.writingLevel.toUByte()
        val text = message.text
        val duration = animator.durationMs(emotion, level, mode, text.length.toUInt()).toLong()
        // Play only once per send or tap; when the card comes back on screen, show the last frame.
        val play = token != message.playedToken && mode != MotionMode.MINIMAL
        message.playedToken = token
        suspend fun draw(t: Long): Boolean = try {
            val pixels = withContext(Dispatchers.Default) {
                animator.render(emotion, level, mode, text, t.toUInt(), side.toUInt(), side.toUInt(), SEED, FrameLayer.STICKER, null)
            }
            bitmap.copyPixelsFromBuffer(ByteBuffer.wrap(pixels))
            tick++
            true
        } catch (cancelled: CancellationException) {
            throw cancelled
        } catch (_: Exception) {
            false // never crash the app for a failed frame; the text is not part of any error
        }
        if (!play) {
            draw(duration)
            return@LaunchedEffect
        }
        val start = SystemClock.elapsedRealtime()
        var frames = 0
        var total = 0.0
        var worst = 0.0
        while (true) {
            val t = (SystemClock.elapsedRealtime() - start).coerceAtMost(duration)
            val begun = System.nanoTime()
            if (!draw(t)) break
            val spent = (System.nanoTime() - begun) / 1e6
            frames++; total += spent; worst = maxOf(worst, spent)
            vm.reportRender(RenderStats(frames, total / frames, worst, side))
            if (t >= duration) break
            if (spent < FRAME_INTERVAL_MS) delay((FRAME_INTERVAL_MS - spent).toLong())
        }
    }
    val replayLabel = stringResource(R.string.replay_button)
    Canvas(
        Modifier.size(STICKER_SIZE)
            .clip(RoundedCornerShape(12.dp))
            .clickable(onClickLabel = replayLabel) { vm.replay(message) }
            .semantics { contentDescription = message.text },
    ) {
        // Reading tick in the draw scope redraws the card for every new frame.
        tick
        drawImage(image, dstSize = IntSize(size.width.toInt(), size.height.toInt()), filterQuality = FilterQuality.High)
    }
}

/** Size of a sticker in the conversation. */
private val STICKER_SIZE = 220.dp

/** Largest sticker side the engine draws, in pixels: 720 px is already sharper than most screens show. */
private const val MAX_STICKER_SIDE = 720
private const val SEED = 1UL
private const val FRAME_INTERVAL_MS = 16.0
