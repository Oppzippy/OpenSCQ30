package com.oppzippy.openscq30.room

import androidx.room.AutoMigration
import androidx.room.Database
import androidx.room.RoomDatabase
import androidx.room.TypeConverters
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetDao
import com.oppzippy.openscq30.ui.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.ui.equalizer.storage.CustomProfileDao

@Database(
    version = 3,
    entities = [
        CustomProfile::class,
        QuickPreset::class,
    ],
    autoMigrations = [
        AutoMigration(from = 1, to = 2),
        AutoMigration(from = 2, to = 3),
    ],
)
@TypeConverters(Converters::class)
abstract class AppDatabase : RoomDatabase() {
    abstract fun equalizerCustomProfileDao(): CustomProfileDao
    abstract fun quickPresetDao(): QuickPresetDao
}
