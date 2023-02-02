package com.oppzippy.openscq30.features.ui.equalizer.models

import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.input.OffsetMapping
import androidx.compose.ui.text.input.TransformedText
import androidx.lifecycle.ViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import java.text.NumberFormat

class EqualizerSliderViewModel : ViewModel() {
    private val prevValue = MutableStateFlow<Byte?>(null)
    private val _displayedText = MutableStateFlow("")
    val displayedText = _displayedText.asStateFlow()

    fun checkForValueChange(value: Byte) {
        if (value != prevValue.value) {
            prevValue.value = value
            _displayedText.value = formatVolume(value)
        }
    }

    fun onValueChange(text: String, callback: (value: Byte) -> Unit) {
        Regex("^(-?)(\\d?)(\\d?)(\\d*)$").matchEntire(text)?.let {
            _displayedText.value = text
            val tens = it.groupValues[2]
            val ones = it.groupValues[3]
            if (tens.isNotEmpty()) {
                try {
                    var value = tens.toInt() * 10
                    var divisor = 10
                    if (ones.isNotEmpty()) {
                        value += ones.toInt()
                        divisor = 1
                    }
                    if (it.groupValues[1].isNotEmpty()) {
                        value *= -1
                    }
                    _displayedText.value = value.div(divisor).toString()
                    prevValue.value = value.toByte()
                    callback(value.toByte())
                } catch (_: NumberFormatException) {
                }
            }
        }
    }

    fun transformText(text: AnnotatedString): TransformedText {
        try {
            val number = text.text.toInt()
            if (number != 0) {
                val newString = text.text.replace(Regex("(\\d)(\\d)"), "$1\\.$2")
                if (newString != text.text) {
                    return TransformedText(
                        AnnotatedString(
                            newString,
                            text.spanStyles,
                            text.paragraphStyles,
                        ),
                        DecimalSeparatorOffsetMapping(text.text.length - 2),
                    )
                }
            }
        } catch (_: NumberFormatException) {
        }
        return TransformedText(text, OffsetMapping.Identity)
    }

    fun formatVolume(volume: Byte): String {
        val format = NumberFormat.getInstance()
        format.minimumFractionDigits = 1
        format.maximumFractionDigits = 1
        format.minimumIntegerDigits = 1
        return format.format(volume / 10F)
    }
}