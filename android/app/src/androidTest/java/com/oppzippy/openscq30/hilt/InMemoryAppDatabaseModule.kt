package com.oppzippy.openscq30.hilt

import android.content.Context
import androidx.room.Room
import com.oppzippy.openscq30.room.AppDatabase
import com.oppzippy.openscq30.room.AppDatabaseModule
import dagger.Module
import dagger.Provides
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import dagger.hilt.testing.TestInstallIn
import javax.inject.Singleton

@Module
@TestInstallIn(
    components = [SingletonComponent::class],
    replaces = [AppDatabaseModule::class],
)
object InMemoryAppDatabaseModule {
    @Provides
    @Singleton
    fun provideAppDatabase(
        @ApplicationContext context: Context,
    ): AppDatabase {
        return Room.inMemoryDatabaseBuilder(context, AppDatabase::class.java)
            .addMigrations(migrations = AppDatabase.migrations.toTypedArray())
            .build()
    }
}
