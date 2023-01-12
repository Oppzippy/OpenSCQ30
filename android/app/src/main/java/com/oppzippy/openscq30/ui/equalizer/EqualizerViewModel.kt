package com.oppzippy.openscq30.ui.equalizer

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.*
import kotlin.math.roundToInt

class EqualizerViewModel : ViewModel() {
    var band100 = MutableStateFlow(0.0F)
    var band200 = MutableStateFlow(0.0F)
    var band400 = MutableStateFlow(0.0F)
    var band800 = MutableStateFlow(0.0F)
    var band1600 = MutableStateFlow(0.0F)
    var band3200 = MutableStateFlow(0.0F)
    var band6400 = MutableStateFlow(0.0F)
    var band12800 = MutableStateFlow(0.0F)

    var bandOffsets =
        combine(band100, band200, band400, band800, band1600, band3200, band6400, band12800) {
            return@combine it.map { value -> value.roundToInt().toByte() }.toByteArray()
        }.stateIn(
            scope = viewModelScope,
            started = SharingStarted.Eagerly,
            initialValue = null,
        )

    fun setBandOffsets(bandOffsets: ByteArray) {
        band100.value = bandOffsets[0].toFloat()
        band200.value = bandOffsets[1].toFloat()
        band400.value = bandOffsets[2].toFloat()
        band800.value = bandOffsets[3].toFloat()
        band1600.value = bandOffsets[4].toFloat()
        band3200.value = bandOffsets[5].toFloat()
        band6400.value = bandOffsets[6].toFloat()
        band12800.value = bandOffsets[7].toFloat()
    }
}

