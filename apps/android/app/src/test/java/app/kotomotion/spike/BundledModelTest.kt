package app.kotomotion.spike

import org.json.JSONObject
import org.junit.Assert.*
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder
import java.io.ByteArrayInputStream
import java.io.File
import java.security.MessageDigest

class BundledModelTest {
    @get:Rule val temporary = TemporaryFolder()

    private fun fixture(revision: String = "one"): MutableMap<String, ByteArray> {
        val assets = mutableMapOf(
            "model/model_q4.onnx" to "graph-$revision".toByteArray(),
            "model/model_q4.onnx_data" to "weights-$revision".toByteArray(),
            "model/tokenizer.json" to "{}".toByteArray(),
            "model/emotion_head.json" to "{}".toByteArray(),
        )
        val hashes = JSONObject()
        assets.forEach { (name, value) ->
            hashes.put(name.removePrefix("model/"), MessageDigest.getInstance("SHA-256").digest(value)
                .joinToString("") { "%02x".format(it.toInt() and 0xff) })
        }
        assets["model/manifest.json"] = JSONObject().put("files", hashes).toString().toByteArray()
        return assets
    }

    @Test fun extractsAllWeightsAndReusesInstalledRevision() {
        val assets = fixture()
        val root = temporary.newFolder()
        val opened = mutableListOf<String>()
        val open = { name: String -> opened.add(name); ByteArrayInputStream(assets.getValue(name)) }
        val model = BundledModel.prepare(root, open)
        assertArrayEquals(assets.getValue("model/model_q4.onnx_data"), File(model, "model_q4.onnx_data").readBytes())
        opened.clear()
        assertEquals(model, BundledModel.prepare(root, open))
        assertEquals(listOf("model/manifest.json"), opened)
        assertTrue(root.listFiles()!!.none { it.name.endsWith(".partial") })
    }

    @Test fun failedCopyDoesNotPublishAPartialModelAndCanRetry() {
        val assets = fixture()
        val root = temporary.newFolder()
        val weights = assets.getValue("model/model_q4.onnx_data")
        assets["model/model_q4.onnx_data"] = "corrupt".toByteArray()
        assertThrows(IllegalStateException::class.java) {
            BundledModel.prepare(root) { ByteArrayInputStream(assets.getValue(it)) }
        }
        assertEquals(0, root.listFiles()!!.size)
        assets["model/model_q4.onnx_data"] = weights
        assertTrue(BundledModel.prepare(root) { ByteArrayInputStream(assets.getValue(it)) }.isDirectory)
    }

    @Test fun rejectsNamesThatEscapePrivateModelStorage() {
        val assets = fixture()
        val manifest = JSONObject(assets.getValue("model/manifest.json").toString(Charsets.UTF_8))
        manifest.getJSONObject("files").put("../escape", "0".repeat(64))
        assets["model/manifest.json"] = manifest.toString().toByteArray()
        val root = temporary.newFolder()
        assertThrows(IllegalArgumentException::class.java) {
            BundledModel.prepare(root) { ByteArrayInputStream(assets.getValue(it)) }
        }
        assertEquals(0, root.listFiles()!!.size)
    }

    @Test fun newApkRevisionReplacesOldModelAndMissingFilesAreRepaired() {
        val root = temporary.newFolder()
        var assets = fixture()
        val first = BundledModel.prepare(root) { ByteArrayInputStream(assets.getValue(it)) }
        assets = fixture("two")
        val second = BundledModel.prepare(root) { ByteArrayInputStream(assets.getValue(it)) }
        assertNotEquals(first, second)
        assertFalse(first.exists())
        File(second, "tokenizer.json").delete()
        assertEquals(second, BundledModel.prepare(root) { ByteArrayInputStream(assets.getValue(it)) })
        assertTrue(File(second, "tokenizer.json").isFile)
    }
}
