package com.oppzippy.openscq30

import android.app.Activity
import androidx.core.app.ComponentActivity
import androidx.lifecycle.LifecycleCoroutineScope
import androidx.lifecycle.lifecycleScope
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.components.ActivityComponent
import dagger.hilt.android.scopes.ActivityScoped

@Module
@InstallIn(ActivityComponent::class)
object CoroutineScopeModule {
    @Provides
    @ActivityScoped
    fun provideLifecycleCoroutineScope(
        activity: Activity,
    ): LifecycleCoroutineScope {
        return (activity as ComponentActivity).lifecycleScope
    }
}
