package com.oppzippy.openscq30.room

import androidx.room.Database
import androidx.room.RoomDatabase
import androidx.room.TypeConverter
import androidx.room.TypeConverters
import com.oppzippy.openscq30.features.ui.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.features.ui.equalizer.storage.CustomProfileDao

@Database(
    version = 1,
    entities = [
        CustomProfile::class,
    ],
)
@TypeConverters(Converters::class)
abstract class AppDatabase : RoomDatabase() {
    abstract fun equalizerCustomProfileDao(): CustomProfileDao
}