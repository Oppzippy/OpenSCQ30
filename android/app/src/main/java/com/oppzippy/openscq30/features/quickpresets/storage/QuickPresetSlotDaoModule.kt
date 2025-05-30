package com.oppzippy.openscq30.features.quickpresets.storage

import com.oppzippy.openscq30.room.AppDatabase
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
class QuickPresetSlotDaoModule {
    @Provides
    @Singleton
    fun provideQuickPresetSlotDao(database: AppDatabase): QuickPresetSlotDao = database.quickPresetSlotDao()
}
