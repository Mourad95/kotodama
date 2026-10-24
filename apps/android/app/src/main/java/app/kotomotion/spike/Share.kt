package app.kotomotion.spike

import android.content.ClipData
import android.content.Context
import android.content.Intent
import androidx.core.content.FileProvider
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import uniffi.kd_ffi.Animator
import uniffi.kd_ffi.MotionMode
import java.io.File

/**
 * Shares a sent message through the system share sheet (ADR-0025): the sticker as a looping GIF, or the
 * plain text when its emotion is not animated. The message leaves the phone only because the user chose
 * an app in the sheet (docs/legal.md L25); the GIF is written to the app cache, replaced by the next
 * share, and handed out through a read-only content URI.
 */
internal object StickerShare {
    /** Side of the shared GIF in pixels: sharp in a chat, a few hundred kilobytes. */
    private const val GIF_SIDE = 480u
    private const val SEED = 1UL

    suspend fun share(context: Context, animator: Animator?, message: ChatMessage, mode: MotionMode) {
        val chooserTitle = context.getString(R.string.share_title)
        if (!message.sticker || animator == null) {
            val send = Intent(Intent.ACTION_SEND).setType("text/plain").putExtra(Intent.EXTRA_TEXT, message.text)
            context.startActivity(Intent.createChooser(send, chooserTitle))
            return
        }
        val file = withContext(Dispatchers.Default) {
            val gif = animator.exportGif(message.emotion, message.writingLevel.toUByte(), mode, message.text, GIF_SIDE, SEED)
            val dir = File(context.cacheDir, "shared")
            dir.deleteRecursively()
            dir.mkdirs()
            File(dir, "kotomotion.gif").apply { writeBytes(gif) }
        }
        val uri = FileProvider.getUriForFile(context, context.packageName + ".share", file)
        val send = Intent(Intent.ACTION_SEND)
            .setType("image/gif")
            .putExtra(Intent.EXTRA_STREAM, uri)
            .addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        send.clipData = ClipData.newRawUri(null, uri)
        context.startActivity(Intent.createChooser(send, chooserTitle))
    }
}
