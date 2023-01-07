package com.oppzippy.openscq30.ui.general

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.NoiseCancelingMode
import kotlinx.coroutines.flow.*

class GeneralViewModel : ViewModel() {
    var ambientSoundModeId = MutableStateFlow(R.id.normal)
    var noiseCancelingModeId = MutableStateFlow(R.id.transport)

    var ambientSoundMode =
        ambientSoundModeId.map {
            return@map when (ambientSoundModeId.value) {
                R.id.normal -> AmbientSoundMode.Normal
                R.id.transparency -> AmbientSoundMode.Transparency
                R.id.noiseCanceling -> AmbientSoundMode.NoiseCanceling
                else -> throw IndexOutOfBoundsException("unexpected ambient sound mode id")
            }
        }.stateIn(
            scope = viewModelScope,
            started = SharingStarted.Eagerly,
            initialValue = AmbientSoundMode.Normal,
        )

    var noiseCancelingMode =
        noiseCancelingModeId.map {
            return@map when (noiseCancelingModeId.value) {
                R.id.transport -> NoiseCancelingMode.Transport
                R.id.indoor -> NoiseCancelingMode.Indoor
                R.id.outdoor -> NoiseCancelingMode.Outdoor
                else -> throw IndexOutOfBoundsException("unexpected ambient sound mode id")
            }
        }.stateIn(
            scope = viewModelScope,
            started = SharingStarted.Eagerly,
            initialValue = NoiseCancelingMode.Outdoor,
        )


    fun setAmbientSoundMode(value: AmbientSoundMode) {
        val selectedId = when (value) {
            AmbientSoundMode.Normal -> R.id.normal
            AmbientSoundMode.Transparency -> R.id.transparency
            AmbientSoundMode.NoiseCanceling -> R.id.noiseCanceling
        }
        ambientSoundModeId.value = selectedId
    }

    fun setNoiseCancelingMode(value: NoiseCancelingMode) {
        val selectedId = when (value) {
            NoiseCancelingMode.Transport -> R.id.transport
            NoiseCancelingMode.Indoor -> R.id.indoor
            NoiseCancelingMode.Outdoor -> R.id.outdoor
        }
        noiseCancelingModeId.value = selectedId
    }
}