package com.oppzippy.openscq30.migrations

import android.content.ContentValues
import android.database.sqlite.SQLiteDatabase
import androidx.room.testing.MigrationTestHelper
import androidx.test.platform.app.InstrumentationRegistry
import com.oppzippy.openscq30.room.AppDatabase
import dagger.hilt.android.testing.HiltAndroidTest
import java.io.IOException
import org.junit.Assert
import org.junit.Rule
import org.junit.Test

@HiltAndroidTest
class Migration5To6Test {
    @get:Rule
    val helper: MigrationTestHelper = MigrationTestHelper(
        InstrumentationRegistry.getInstrumentation(),
        AppDatabase::class.java,
    )

    @Test
    @Throws(IOException::class)
    fun migrate5To6() {
        val dbName = "migration_5_to_6_test_db"
        val profiles = mapOf(
            Pair("profile1", byteArrayOf(-120, 0, 120, -60, 60, 5, 15, 4)),
        )
        helper.createDatabase(dbName, 5).apply {
            // Database has schema version 1. Insert some data using SQL queries.
            // You can't use DAO classes because they expect the latest schema.
            profiles.forEach { (name, values) ->
                insert(
                    "equalizer_custom_profile",
                    SQLiteDatabase.CONFLICT_NONE,
                    ContentValues().apply {
                        put("name", name)
                        put("`values`", values)
                    },
                )
            }

            // Prepare for the next version.
            close()
        }

        // Re-open the database with version 2 and provide
        // MIGRATION_1_2 as the migration process.
        val db = helper.runMigrationsAndValidate(dbName, 6, true, AppDatabase.Migration5To6)

        // MigrationTestHelper automatically verifies the schema changes,
        // but you need to validate that the data was migrated properly.
        val cursor = db.query("SELECT * FROM custom_equalizer_profile ORDER BY name ASC")
        var numResults = 0
        while (cursor.moveToNext()) {
            numResults++
            val values = profiles[cursor.getString(cursor.getColumnIndexOrThrow("name"))]!!
            Assert.assertEquals(
                values[0].toDouble() / 10,
                cursor.getDouble(cursor.getColumnIndexOrThrow("band100")),
                0.0001,
            )
        }
        Assert.assertEquals(profiles.size, numResults)
    }
}
