package com.oppzippy.openscq30.ui.devicesettings.screens

import com.oppzippy.openscq30.lib.wrapper.Range
import com.oppzippy.openscq30.lib.wrapper.Select
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.toValue
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class SoundModeChangesTest {
    private fun select(value: String, vararg options: String) =
        Setting.SelectSetting(Select(options.toList(), options.toList()), value)

    /** Settings shaped like the Soundcore Liberty 5 (A3957) */
    private fun liberty5(
        ambientSoundMode: String = "NoiseCanceling",
        noiseCancelingMode: String = "Manual",
        manualNoiseCanceling: Int = 5,
        transportationMode: String = "Plane",
    ): Map<String, Setting> = mapOf(
        "ambientSoundMode" to select(ambientSoundMode, "NoiseCanceling", "Transparency", "Normal"),
        "transparencyMode" to select("FullyTransparent", "FullyTransparent", "VocalMode"),
        "noiseCancelingMode" to select(noiseCancelingMode, "Manual", "Transportation"),
        "manualNoiseCanceling" to Setting.I32RangeSetting(Range(1, 5, 1), manualNoiseCanceling),
        "transportationMode" to select(transportationMode, "Plane", "Car"),
        "windNoiseSuppression" to Setting.ToggleSetting(false),
        "windNoiseDetected" to Setting.InformationSetting("false", "No"),
    )

    @Test
    fun choosingCarFromTransparencySwitchesToNoiseCancelingTransportation() {
        val changes = soundModeChanges(
            "transportationMode",
            "Car".toValue(),
            liberty5(ambientSoundMode = "Transparency"),
        )
        assertEquals(
            listOf(
                "ambientSoundMode" to "NoiseCanceling".toValue(),
                "noiseCancelingMode" to "Transportation".toValue(),
                "transportationMode" to "Car".toValue(),
            ),
            changes,
        )
    }

    @Test
    fun choosingLevelWhileInTransportationModeSwitchesToManual() {
        val changes = soundModeChanges(
            "manualNoiseCanceling",
            3.toValue(),
            liberty5(noiseCancelingMode = "Transportation"),
        )
        assertEquals(
            listOf("noiseCancelingMode" to "Manual".toValue(), "manualNoiseCanceling" to 3.toValue()),
            changes,
        )
    }

    @Test
    fun alreadyActiveSettingIsChangedAlone() {
        val changes = soundModeChanges("manualNoiseCanceling", 2.toValue(), liberty5())
        assertEquals(listOf("manualNoiseCanceling" to 2.toValue()), changes)
    }

    @Test
    fun choosingVocalModeFromNoiseCancelingSwitchesToTransparency() {
        val changes = soundModeChanges("transparencyMode", "VocalMode".toValue(), liberty5())
        assertEquals(
            listOf("ambientSoundMode" to "Transparency".toValue(), "transparencyMode" to "VocalMode".toValue()),
            changes,
        )
    }

    @Test
    fun relevanceMatchesCurrentMode() {
        val settings = liberty5(ambientSoundMode = "NoiseCanceling", noiseCancelingMode = "Manual")
        assertTrue(soundModeRelevance("manualNoiseCanceling", settings).isActive)
        assertFalse(soundModeRelevance("transportationMode", settings).isActive)
        assertFalse(soundModeRelevance("transparencyMode", settings).isActive)
        assertTrue(soundModeRelevance("windNoiseSuppression", settings).isVisible)
        assertFalse(soundModeRelevance("windNoiseDetected", settings).isVisible)
    }

    @Test
    fun windNoiseSuppressionIsHiddenInNormalMode() {
        val settings = liberty5(ambientSoundMode = "Normal")
        assertFalse(soundModeRelevance("windNoiseSuppression", settings).isVisible)
        assertFalse(soundModeRelevance("manualNoiseCanceling", settings).isActive)
    }

    @Test
    fun parentsWithoutTheRelevantOptionAreIgnored() {
        // e.g. Life Q30 style noise canceling modes, which have no "Manual" option
        val settings = mapOf(
            "ambientSoundMode" to select("NoiseCanceling", "NoiseCanceling", "Transparency", "Normal"),
            "noiseCancelingMode" to select("Indoor", "Transport", "Indoor", "Outdoor", "Custom"),
            "manualNoiseCanceling" to Setting.I32RangeSetting(Range(1, 5, 1), 1),
        )
        assertEquals(
            listOf("manualNoiseCanceling" to 2.toValue()),
            soundModeChanges("manualNoiseCanceling", 2.toValue(), settings),
        )
    }

    @Test
    fun settingsAreGroupedByMode() {
        val ordered = orderSoundModeSettings(liberty5().toList()).map { it.first }
        assertEquals(
            listOf(
                "noiseCancelingMode",
                "manualNoiseCanceling",
                "transportationMode",
                "transparencyMode",
                "ambientSoundMode",
                "windNoiseSuppression",
                "windNoiseDetected",
            ),
            ordered,
        )
    }
}
