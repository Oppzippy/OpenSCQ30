package com.oppzippy.openscq30.lib.wrapper

import com.oppzippy.openscq30.lib.bindings.EqualizerConfiguration
import com.oppzippy.openscq30.lib.bindings.SoundModes
import com.oppzippy.openscq30.lib.bindings.StateUpdatePacket
import kotlin.jvm.optionals.getOrNull

data class SoundcoreDeviceState(
    val featureFlags: Int,
//    val battery: Battery,
    val equalizerConfiguration: EqualizerConfiguration,
    val soundModes: SoundModes?,
//    val ageRange: AgeRange?,
//    val customHearId: HearId?,
//    val customButtonModel: CustomButtonModel?,
    val firmwareVersion: String?,
    val serialNumber: String?,
) {
    // used for static extension methods in tests
    companion object;

    override fun equals(other: Any?): Boolean {
        if (other !is SoundcoreDeviceState) return false

        return featureFlags == other.featureFlags &&
            equalizerConfiguration == other.equalizerConfiguration &&
            // overly complicated equality check since they are both nullable
            (soundModes == other.soundModes || other.soundModes?.let { soundModes?.innerEquals(it) } ?: false) &&
            firmwareVersion == other.firmwareVersion &&
            serialNumber == other.serialNumber
    }
}

fun StateUpdatePacket.toSoundcoreDeviceState(): SoundcoreDeviceState {
    return SoundcoreDeviceState(
        featureFlags = featureFlags(),
        equalizerConfiguration = equalizerConfiguration(),
        soundModes = soundModes().getOrNull(),
        firmwareVersion = firmwareVersion().getOrNull(),
        serialNumber = serialNumber().getOrNull(),
    )
}
