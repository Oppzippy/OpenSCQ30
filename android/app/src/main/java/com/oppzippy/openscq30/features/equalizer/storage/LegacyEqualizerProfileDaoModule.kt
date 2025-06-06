package com.oppzippy.openscq30.features.equalizer.storage

import com.oppzippy.openscq30.room.AppDatabase
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
class LegacyEqualizerProfileDaoModule {
    @Provides
    @Singleton
    fun provideCustomProfileDao(database: AppDatabase): LegacyEqualizerProfileDao = database.equalizerCustomProfileDao()
}
