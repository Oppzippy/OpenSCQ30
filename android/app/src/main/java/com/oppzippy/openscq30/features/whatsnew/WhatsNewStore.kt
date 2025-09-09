package com.oppzippy.openscq30.features.whatsnew

import android.content.Context
import androidx.core.content.edit
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

class WhatsNewStore @Inject constructor(@ApplicationContext context: Context) {
    private val preferences = context.getSharedPreferences("whats-new", Context.MODE_PRIVATE)

    private val _version2BreakingChangesMessageShown =
        MutableStateFlow(preferences.getBoolean("version2BreakingChangesMessageShown", false))
    val version2BreakingChangesMessageShown = _version2BreakingChangesMessageShown.asStateFlow()

    fun setVersion2BreakingChangesMessageShown(shown: Boolean) {
        _version2BreakingChangesMessageShown.value = shown
        preferences.edit { putBoolean("version2BreakingChangesMessageShown", shown) }
    }
}
