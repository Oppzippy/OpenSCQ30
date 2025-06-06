package com.oppzippy.openscq30.lib.hilt

import android.content.Context
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.newSession
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton
import kotlin.io.path.Path
import kotlin.io.path.pathString
import kotlinx.coroutines.runBlocking

@Module
@InstallIn(SingletonComponent::class)
object OpenSCQ30SessionModule {
    @Provides
    @Singleton
    fun provideOpenSCQ30Session(@ApplicationContext context: Context): OpenScq30Session {
        val dataDir = Path(context.applicationInfo.dataDir)
        return runBlocking { newSession(dataDir.resolve("openscq30_lib.sqlite").pathString) }
    }
}
