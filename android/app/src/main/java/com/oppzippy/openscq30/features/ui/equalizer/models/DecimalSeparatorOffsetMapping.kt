package com.oppzippy.openscq30.features.ui.equalizer.models

import androidx.compose.ui.text.input.OffsetMapping

class DecimalSeparatorOffsetMapping(private val decimalIndex: Int): OffsetMapping {
    override fun originalToTransformed(offset: Int): Int {
        return if (offset > decimalIndex) {
            offset + 1
        } else {
            offset
        }
    }

    override fun transformedToOriginal(offset: Int): Int {
        return if (offset > decimalIndex + 1) {
            offset - 1
        } else {
            offset
        }
    }
}