package com.oppzippy.openscq30.migrations

import android.content.ContentValues
import android.database.sqlite.SQLiteDatabase
import androidx.room.testing.MigrationTestHelper
import androidx.test.platform.app.InstrumentationRegistry
import com.oppzippy.openscq30.room.AppDatabase
import dagger.hilt.android.testing.HiltAndroidTest
import java.io.IOException
import java.util.UUID
import org.junit.Assert
import org.junit.Rule
import org.junit.Test

@HiltAndroidTest
class Migration9To10Test {
    @get:Rule
    val helper: MigrationTestHelper = MigrationTestHelper(
        InstrumentationRegistry.getInstrumentation(),
        AppDatabase::class.java,
    )

    @Test
    @Throws(IOException::class)
    fun idsGetAssigned() {
        val dbName = "migration_9_to_10_test_db"
        helper.createDatabase(dbName, 9).apply {
            insert(
                "quick_preset",
                SQLiteDatabase.CONFLICT_NONE,
                ContentValues().apply {
                    put("deviceBleServiceUuid", UUID.randomUUID().toString())
                    put("`index`", 0)
                },
            )
            insert(
                "quick_preset",
                SQLiteDatabase.CONFLICT_NONE,
                ContentValues().apply {
                    put("deviceBleServiceUuid", UUID.randomUUID().toString())
                    put("`index`", 1)
                },
            )

            // Prepare for the next version.
            close()
        }

        val db = helper.runMigrationsAndValidate(dbName, 10, true)

        val cursor = db.query("SELECT * FROM quick_preset")
        val ids = hashSetOf(1, 2)
        while (cursor.moveToNext()) {
            val id = cursor.getInt(cursor.getColumnIndexOrThrow("id"))
            ids.remove(id)
        }
        Assert.assertEquals(emptySet<Int>(), ids)
    }
}
