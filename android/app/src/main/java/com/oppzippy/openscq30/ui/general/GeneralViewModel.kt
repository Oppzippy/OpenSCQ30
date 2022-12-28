package com.oppzippy.openscq30.ui.general

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.models.AmbientSoundMode
import com.oppzippy.openscq30.models.NoiseCancelingMode
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.runBlocking

class GeneralViewModel : ViewModel() {
    var ambientSoundModeId = MutableStateFlow(R.id.normal)
    var noiseCancelingModeId = MutableStateFlow(R.id.transport)

    var ambientSoundMode =
        ambientSoundModeId.map {
            return@map when (ambientSoundModeId.value) {
                R.id.normal -> AmbientSoundMode.NORMAL
                R.id.transparency -> AmbientSoundMode.TRANSPARENCY
                R.id.noiseCanceling -> AmbientSoundMode.NOISE_CANCELING
                else -> throw IndexOutOfBoundsException("unexpected ambient sound mode id")
            }
        }.stateIn(
            scope = viewModelScope,
            started = SharingStarted.Eagerly,
            initialValue = AmbientSoundMode.NORMAL,
        )

    var noiseCancelingMode =
        noiseCancelingModeId.map {
            return@map when (noiseCancelingModeId.value) {
                R.id.transport -> NoiseCancelingMode.TRANSPORT
                R.id.indoor -> NoiseCancelingMode.INDOOR
                R.id.outdoor -> NoiseCancelingMode.OUTDOOR
                else -> throw IndexOutOfBoundsException("unexpected ambient sound mode id")
            }
        }.stateIn(
            scope = viewModelScope,
            started = SharingStarted.Eagerly,
            initialValue = NoiseCancelingMode.OUTDOOR,
        )


    fun setAmbientSoundMode(value: AmbientSoundMode) {
        val selectedId = when (value) {
            AmbientSoundMode.NORMAL -> R.id.normal
            AmbientSoundMode.TRANSPARENCY -> R.id.transparency
            AmbientSoundMode.NOISE_CANCELING -> R.id.noiseCanceling
        }
        ambientSoundModeId.value = selectedId
    }

    fun setNoiseCancelingMode(value: NoiseCancelingMode) {
        val selectedId = when (value) {
            NoiseCancelingMode.TRANSPORT -> R.id.transport
            NoiseCancelingMode.INDOOR -> R.id.indoor
            NoiseCancelingMode.OUTDOOR -> R.id.outdoor
        }
        noiseCancelingModeId.value = selectedId
    }
}