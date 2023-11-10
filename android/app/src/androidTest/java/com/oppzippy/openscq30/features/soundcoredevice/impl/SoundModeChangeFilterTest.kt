package com.oppzippy.openscq30.features.soundcoredevice.impl

import com.oppzippy.openscq30.lib.bindings.AmbientSoundMode
import com.oppzippy.openscq30.lib.bindings.CustomNoiseCanceling
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingMode
import com.oppzippy.openscq30.lib.bindings.NoiseCancelingModeType
import com.oppzippy.openscq30.lib.bindings.SoundModeProfile
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.bindings.TransparencyMode
import com.oppzippy.openscq30.lib.bindings.TransparencyModeType
import com.oppzippy.openscq30.lib.extensions.structures.copy
import dagger.hilt.android.testing.HiltAndroidTest
import org.junit.Assert
import org.junit.Test

@HiltAndroidTest
class SoundModeChangeFilterTest {
    private val soundModes = SoundModes(
        AmbientSoundMode.Normal,
        NoiseCancelingMode.Indoor,
        TransparencyMode.VocalMode,
        CustomNoiseCanceling(0),
    )

    @Test
    fun preventsNoiseCancelingMode() {
        val filtered = filterSoundModeChanges(
            SoundModeProfile(NoiseCancelingModeType.None, TransparencyModeType.Basic),
            soundModes,
            soundModes.copy(ambientSoundMode = AmbientSoundMode.NoiseCanceling),
        )
        Assert.assertEquals(soundModes.ambientSoundMode(), filtered.ambientSoundMode())
    }

    @Test
    fun preventsTransportMode() {
        val filtered = filterSoundModeChanges(
            SoundModeProfile(NoiseCancelingModeType.None, TransparencyModeType.Basic),
            soundModes,
            soundModes.copy(noiseCancelingMode = NoiseCancelingMode.Transport),
        )
        Assert.assertEquals(soundModes.noiseCancelingMode(), filtered.noiseCancelingMode())
    }

    @Test
    fun preventsCustomNoiseCancelingMode() {
        val filtered = filterSoundModeChanges(
            SoundModeProfile(NoiseCancelingModeType.Basic, TransparencyModeType.Basic),
            soundModes,
            soundModes.copy(noiseCancelingMode = NoiseCancelingMode.Custom),
        )
        Assert.assertEquals(soundModes.noiseCancelingMode(), filtered.noiseCancelingMode())
    }

    @Test
    fun preventsFullyTransparent() {
        val filtered = filterSoundModeChanges(
            SoundModeProfile(NoiseCancelingModeType.None, TransparencyModeType.Basic),
            soundModes,
            soundModes.copy(transparencyMode = TransparencyMode.FullyTransparent),
        )
        Assert.assertEquals(soundModes.transparencyMode(), filtered.transparencyMode())
    }

    @Test
    fun preventsCustomNoiseCanceling1() {
        val filtered = filterSoundModeChanges(
            SoundModeProfile(NoiseCancelingModeType.None, TransparencyModeType.Basic),
            soundModes,
            soundModes.copy(customNoiseCanceling = CustomNoiseCanceling(1)),
        )
        Assert.assertEquals(soundModes.customNoiseCanceling(), filtered.customNoiseCanceling())
    }

    @Test
    fun preventsNothingWithAllFlags() {
        val newSoundModes = SoundModes(
            AmbientSoundMode.NoiseCanceling,
            NoiseCancelingMode.Custom,
            TransparencyMode.FullyTransparent,
            CustomNoiseCanceling(1),
        )
        val filtered = filterSoundModeChanges(
            SoundModeProfile(NoiseCancelingModeType.Custom, TransparencyModeType.Custom),
            soundModes,
            newSoundModes,
        )
        Assert.assertEquals(newSoundModes, filtered)
    }
}
