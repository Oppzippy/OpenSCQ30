package com.oppzippy.openscq30.room

import android.content.ContentValues
import android.database.sqlite.SQLiteDatabase
import androidx.room.AutoMigration
import androidx.room.Database
import androidx.room.RenameColumn
import androidx.room.RenameTable
import androidx.room.RoomDatabase
import androidx.room.TypeConverters
import androidx.room.migration.AutoMigrationSpec
import androidx.room.migration.Migration
import androidx.sqlite.db.SupportSQLiteDatabase
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.quickpresets.storage.FallbackQuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetDao

@Database(
    version = 8,
    entities = [
        CustomProfile::class,
        FallbackQuickPreset::class,
        QuickPreset::class,
    ],
    autoMigrations = [
        AutoMigration(from = 1, to = 2),
        AutoMigration(from = 2, to = 3),
        AutoMigration(from = 3, to = 4, spec = AppDatabase.AutoMigration3To4::class),
        AutoMigration(from = 4, to = 5),
        AutoMigration(from = 6, to = 7, spec = AppDatabase.AutoMigration6To7::class),
        AutoMigration(from = 7, to = 8, spec = AppDatabase.AutoMigration7To8::class),
    ],
)
@TypeConverters(Converters::class)
abstract class AppDatabase : RoomDatabase() {

    abstract fun equalizerCustomProfileDao(): CustomProfileDao
    abstract fun quickPresetDao(): QuickPresetDao

    @RenameColumn(
        tableName = "quick_preset",
        fromColumnName = "equalizerProfileName",
        toColumnName = "customEqualizerProfileName",
    )
    class AutoMigration3To4 : AutoMigrationSpec

    @RenameTable(
        fromTableName = "quick_preset",
        toTableName = "fallback_quick_preset",
    )
    @RenameColumn(
        tableName = "quick_preset",
        fromColumnName = "id",
        toColumnName = "index",
    )
    class AutoMigration6To7 : AutoMigrationSpec

    @RenameTable(
        fromTableName = "device_quick_preset",
        toTableName = "quick_preset",
    )
    class AutoMigration7To8 : AutoMigrationSpec

    companion object {
        val migrations = listOf(Migration5To6)
    }

    object Migration5To6 : Migration(5, 6) {
        override fun migrate(db: SupportSQLiteDatabase) {
            //language=RoomSql
            db.execSQL("ALTER TABLE equalizer_custom_profile RENAME TO custom_equalizer_profile_pre_migration")
            //language=RoomSql
            db.execSQL(
                """CREATE TABLE `custom_equalizer_profile` (
                    name TEXT PRIMARY KEY NOT NULL,
                    band100 REAL NOT NULL,
                    band200 REAL NOT NULL,
                    band400 REAL NOT NULL,
                    band800 REAL NOT NULL,
                    band1600 REAL NOT NULL,
                    band3200 REAL NOT NULL,
                    band6400 REAL NOT NULL,
                    band12800 REAL NOT NULL
                )
                """.trimMargin(),
            )
            //language=RoomSql
            db.execSQL(
                """CREATE UNIQUE INDEX index_custom_equalizer_profile_band100_band200_band400_band800_band1600_band3200_band6400_band12800 ON custom_equalizer_profile (
                    band100,
                    band200,
                    band400,
                    band800,
                    band1600,
                    band3200,
                    band6400,
                    band12800
                )
                """.trimMargin(),
            )

            db.beginTransaction()
            try {
                //language=RoomSql
                val cursor =
                    db.query("SELECT name, `values` FROM custom_equalizer_profile_pre_migration")
                while (cursor.moveToNext()) {
                    val name = cursor.getString(0)
                    val byteValues = cursor.getBlob(1)
                    val doubleValues = byteValues.map { it.toDouble() / 10.0 }
                    db.insert(
                        "custom_equalizer_profile",
                        SQLiteDatabase.CONFLICT_REPLACE,
                        ContentValues().apply {
                            put("name", name)
                            put("band100", doubleValues[0])
                            put("band200", doubleValues[1])
                            put("band400", doubleValues[2])
                            put("band800", doubleValues[3])
                            put("band1600", doubleValues[4])
                            put("band3200", doubleValues[5])
                            put("band6400", doubleValues[6])
                            put("band12800", doubleValues[7])
                        },
                    )
                }
                db.setTransactionSuccessful()
            } finally {
                db.endTransaction()
            }
            //language=RoomSql
            db.execSQL("DROP TABLE custom_equalizer_profile_pre_migration")
        }
    }
}
