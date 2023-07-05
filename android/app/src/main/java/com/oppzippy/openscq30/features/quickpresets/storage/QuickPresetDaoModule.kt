package com.oppzippy.openscq30.features.quickpresets.storage

import com.oppzippy.openscq30.room.AppDatabase
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object QuickPresetDaoModule {
    @Provides
    @Singleton
    fun provideQuickPresetDao(database: AppDatabase): QuickPresetDao {
        return database.quickPresetDao()
    }
}
