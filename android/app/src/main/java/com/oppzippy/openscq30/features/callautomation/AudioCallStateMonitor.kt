package com.oppzippy.openscq30.features.callautomation

import android.content.Context
import android.media.AudioManager
import android.os.Build
import androidx.annotation.RequiresApi
import androidx.core.content.ContextCompat
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.flow

class AudioCallStateMonitor(context: Context) {
    private val audioManager = context.getSystemService(AudioManager::class.java)
    private val callbackExecutor = ContextCompat.getMainExecutor(context)

    fun states(): Flow<Boolean> = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
        listenerStates()
    } else {
        pollingStates()
    }.distinctUntilChanged()

    @RequiresApi(Build.VERSION_CODES.S)
    private fun listenerStates(): Flow<Boolean> = callbackFlow {
        val listener = AudioManager.OnModeChangedListener { mode ->
            trySend(isCallAudioMode(mode))
        }
        audioManager.addOnModeChangedListener(callbackExecutor, listener)
        trySend(isCallAudioMode(audioManager.mode))
        awaitClose { audioManager.removeOnModeChangedListener(listener) }
    }

    private fun pollingStates(): Flow<Boolean> = flow {
        while (true) {
            emit(isCallAudioMode(audioManager.mode))
            delay(500)
        }
    }
}

internal fun isCallAudioMode(mode: Int): Boolean =
    mode == AudioManager.MODE_IN_CALL || mode == AudioManager.MODE_IN_COMMUNICATION
