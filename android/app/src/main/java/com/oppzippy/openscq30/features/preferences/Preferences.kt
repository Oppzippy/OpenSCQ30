package com.oppzippy.openscq30.features.preferences

import android.content.Context
import androidx.core.content.edit
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject

class Preferences @Inject constructor(@ApplicationContext context: Context) {
    private val preferences = context.getSharedPreferences("preferences", Context.MODE_PRIVATE)

    var autoConnect: Boolean
        get() {
            return preferences.getBoolean("autoConnect", false)
        }
        set(value) {
            preferences.edit {
                putBoolean("autoConnect", value)
            }
        }
}
