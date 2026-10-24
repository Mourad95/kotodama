package app.kotomotion.spike

import org.json.JSONObject
import java.io.File
import java.io.InputStream
import java.security.MessageDigest

/** Extracts only a checksum-verified APK model, off the UI thread, into app-private storage. */
internal object BundledModel {
    private val fileName = Regex("[A-Za-z0-9][A-Za-z0-9._-]*")
    private val sha256 = Regex("[a-f0-9]{64}")

    fun prepare(root: File, openAsset: (String) -> InputStream): File {
        val manifest = openAsset("model/manifest.json").use { it.readBytes() }
        val identity = digest(manifest)
        val files = JSONObject(manifest.toString(Charsets.UTF_8)).getJSONObject("files")
        val entries = files.keys().asSequence().associateWith { name ->
            require(fileName.matches(name)) { "Invalid model filename" }
            files.getString(name).also { require(sha256.matches(it)) { "Invalid model checksum" } }
        }
        require(entries.keys.containsAll(listOf("model_q4.onnx", "tokenizer.json", "emotion_head.json"))) {
            "Incomplete model manifest"
        }
        require("manifest.json" !in entries) { "Model manifest cannot reference itself" }
        check(root.isDirectory || root.mkdirs()) { "Cannot create private model storage" }
        val target = File(root, identity)
        val installed = File(target, "manifest.json")
        if (installed.isFile && installed.readBytes().contentEquals(manifest) &&
            entries.keys.all { File(target, it).isFile }) return target

        // Never leave a partially copied model looking installed after interruption or disk failure.
        val staging = File(root, "$identity.partial")
        staging.deleteRecursively()
        check(staging.mkdirs()) { "Cannot stage model" }
        try {
            entries.forEach { (name, expected) ->
                val hash = MessageDigest.getInstance("SHA-256")
                openAsset("model/$name").use { input ->
                    File(staging, name).outputStream().buffered().use { output ->
                        val buffer = ByteArray(64 * 1024)
                        while (true) {
                            val count = input.read(buffer)
                            if (count < 0) break
                            hash.update(buffer, 0, count)
                            output.write(buffer, 0, count)
                        }
                    }
                }
                check(hex(hash.digest()) == expected) { "Model checksum mismatch" }
            }
            File(staging, "manifest.json").writeBytes(manifest)
            if (target.exists()) check(target.deleteRecursively()) { "Cannot replace incomplete model" }
            check(staging.renameTo(target)) { "Cannot install model" }
        } finally {
            staging.deleteRecursively()
        }
        // Old packaged revisions contain no user data and can be replaced by this verified revision.
        root.listFiles()?.filter { it.isDirectory && it != target && sha256.matches(it.name) }
            ?.forEach { it.deleteRecursively() }
        return target
    }

    private fun digest(bytes: ByteArray): String = hex(MessageDigest.getInstance("SHA-256").digest(bytes))
    private fun hex(bytes: ByteArray): String = bytes.joinToString("") { "%02x".format(it.toInt() and 0xff) }
}
