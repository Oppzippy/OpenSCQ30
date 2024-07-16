package com.oppzippy.openscq30.migrations

import android.content.ContentValues
import android.database.sqlite.SQLiteDatabase
import androidx.room.testing.MigrationTestHelper
import androidx.test.platform.app.InstrumentationRegistry
import com.oppzippy.openscq30.room.AppDatabase
import dagger.hilt.android.testing.HiltAndroidTest
import org.junit.Assert
import org.junit.Rule
import org.junit.Test
import java.io.IOException

@HiltAndroidTest
class Migration8To9Test {
    @get:Rule
    val helper: MigrationTestHelper = MigrationTestHelper(
        InstrumentationRegistry.getInstrumentation(),
        AppDatabase::class.java,
    )

    @Test
    @Throws(IOException::class)
    fun migrate8To9() {
        val dbName = "migration_8_to_9_test_db"
        val profiles = mapOf(
            Pair("profile1", listOf(-12.0, 0.0, 12.0, -6.0, 6.0, 0.5, 1.500001, 0.399999)),
        )
        val newProfiles = mapOf(
            Pair("profile1", listOf(-120, 0, 120, -60, 60, 5, 15, 4)),
        )
        helper.createDatabase(dbName, 8).apply {
            profiles.forEach { (name, values) ->
                insert(
                    "custom_equalizer_profile",
                    SQLiteDatabase.CONFLICT_NONE,
                    ContentValues().apply {
                        put("name", name)
                        put("band100", values[0])
                        put("band200", values[1])
                        put("band400", values[2])
                        put("band800", values[3])
                        put("band1600", values[4])
                        put("band3200", values[5])
                        put("band6400", values[6])
                        put("band12800", values[7])
                    },
                )
            }

            // Prepare for the next version.
            close()
        }

        val db = helper.runMigrationsAndValidate(dbName, 9, true, AppDatabase.Migration8To9)

        val cursor = db.query("SELECT * FROM custom_equalizer_profile")
        var numResults = 0
        while (cursor.moveToNext()) {
            numResults++
            val name = cursor.getString(cursor.getColumnIndexOrThrow("name"))
            val newValues = newProfiles[name]!!
            Assert.assertEquals(
                newValues[0],
                cursor.getInt(cursor.getColumnIndexOrThrow("band100")),
            )
            Assert.assertEquals(
                newValues[1],
                cursor.getInt(cursor.getColumnIndexOrThrow("band200")),
            )
            Assert.assertEquals(
                newValues[2],
                cursor.getInt(cursor.getColumnIndexOrThrow("band400")),
            )
            Assert.assertEquals(
                newValues[3],
                cursor.getInt(cursor.getColumnIndexOrThrow("band800")),
            )
            Assert.assertEquals(
                newValues[4],
                cursor.getInt(cursor.getColumnIndexOrThrow("band1600")),
            )
            Assert.assertEquals(
                newValues[5],
                cursor.getInt(cursor.getColumnIndexOrThrow("band3200")),
            )
            Assert.assertEquals(
                newValues[6],
                cursor.getInt(cursor.getColumnIndexOrThrow("band6400")),
            )
            Assert.assertEquals(
                newValues[7],
                cursor.getInt(cursor.getColumnIndexOrThrow("band12800")),
            )
        }
        Assert.assertEquals(profiles.size, numResults)
    }
}
