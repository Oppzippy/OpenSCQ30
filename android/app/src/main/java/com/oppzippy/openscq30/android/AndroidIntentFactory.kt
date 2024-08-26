package com.oppzippy.openscq30.android

import android.content.Context
import android.content.Intent
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

interface IntentFactory {
    operator fun invoke(context: Context, cls: Class<*>): Intent
}

class AndroidIntentFactory : IntentFactory {
    override fun invoke(context: Context, cls: Class<*>): Intent = Intent(context, cls)
}

@Module
@InstallIn(SingletonComponent::class)
object IntentFactoryModule {
    @Provides
    @Singleton
    fun provideLifecycleCoroutineScope(): IntentFactory = AndroidIntentFactory()
}
