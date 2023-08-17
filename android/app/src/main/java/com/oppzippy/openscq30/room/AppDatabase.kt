package com.oppzippy.openscq30.room

import androidx.room.AutoMigration
import androidx.room.Database
import androidx.room.RenameColumn
import androidx.room.RoomDatabase
import androidx.room.TypeConverters
import androidx.room.migration.AutoMigrationSpec
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetDao

@Database(
    version = 5,
    entities = [
        CustomProfile::class,
        QuickPreset::class,
    ],
    autoMigrations = [
        AutoMigration(from = 1, to = 2),
        AutoMigration(from = 2, to = 3),
        AutoMigration(
            from = 3,
            to = 4,
            spec = AppDatabase.CustomEqualizerProfileNameMigration::class,
        ),
        AutoMigration(from = 4, to = 5),
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
    class CustomEqualizerProfileNameMigration : AutoMigrationSpec
}
