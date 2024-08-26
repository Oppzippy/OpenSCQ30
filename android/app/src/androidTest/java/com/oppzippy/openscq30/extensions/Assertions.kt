package com.oppzippy.openscq30.extensions

import androidx.compose.ui.semantics.SemanticsProperties
import androidx.compose.ui.test.SemanticsMatcher
import androidx.compose.ui.test.SemanticsNodeInteraction
import androidx.compose.ui.test.assert
import kotlin.math.absoluteValue

fun SemanticsNodeInteraction.assertRangeValueApproxEquals(value: Float): SemanticsNodeInteraction = assert(
    SemanticsMatcher("range value approx eq $value") {
        (it.config[SemanticsProperties.ProgressBarRangeInfo].current - value).absoluteValue < 0.0001f
    },
)
