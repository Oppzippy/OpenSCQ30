package com.oppzippy.openscq30.features.soundcoredevice.impl

import android.util.Log
import com.oppzippy.openscq30.lib.bindings.BatteryChargingUpdatePacket
import com.oppzippy.openscq30.lib.bindings.BatteryLevelUpdatePacket
import com.oppzippy.openscq30.lib.bindings.ChineseVoicePromptStateUpdatePacket
import com.oppzippy.openscq30.lib.bindings.FirmwareVersionUpdatePacket
import com.oppzippy.openscq30.lib.bindings.InboundPacket
import com.oppzippy.openscq30.lib.bindings.LdacStateUpdatePacket
import com.oppzippy.openscq30.lib.bindings.SetEqualizerOkPacket
import com.oppzippy.openscq30.lib.bindings.SetEqualizerWithDrcOkPacket
import com.oppzippy.openscq30.lib.bindings.SetSoundModeOkPacket
import com.oppzippy.openscq30.lib.bindings.SoundModeUpdatePacket
import com.oppzippy.openscq30.lib.bindings.StateUpdatePacket
import com.oppzippy.openscq30.lib.bindings.TwsStatusUpdatePacket
import kotlin.jvm.optionals.getOrNull

sealed class Packet {
    companion object {
        fun fromBytes(input: ByteArray): Packet? {
            val inboundPacket = try {
                InboundPacket(input)
            } catch (ex: Exception) {
                Log.w("Packet", "received unknown or invalid packet", ex)
                return null
            }
            return inboundPacket.soundModeUpdate().getOrNull()?.let { SoundModeUpdate(it) }
                ?: inboundPacket.stateUpdate().getOrNull()?.let { StateUpdate(it) }
                ?: inboundPacket.setSoundModeOk().getOrNull()?.let { SetSoundModeOk(it) }
                ?: inboundPacket.setEqualizerOk().getOrNull()?.let { SetEqualizerOk(it) }
                ?: inboundPacket.setEqualizerWithDrcOk().getOrNull()
                    ?.let { SetEqualizerWithDrcOk(it) }
                ?: inboundPacket.batteryChargingUpdate().getOrNull()
                    ?.let { BatteryChargingUpdate(it) }
                ?: inboundPacket.batteryLevelUpdate().getOrNull()?.let { BatteryLevelUpdate(it) }
                ?: inboundPacket.chineseVoicePromptStateUpdate().getOrNull()
                    ?.let { ChineseVoicePromptStateUpdate(it) }
                ?: inboundPacket.firmwareVersionUpdate().getOrNull()
                    ?.let { FirmwareVersionUpdate(it) }
                ?: inboundPacket.ldacStateUpdate().getOrNull()
                    ?.let { LdacStateUpdate(it) }
                ?: inboundPacket.twsStatusUpdate().getOrNull()
                    ?.let { TwsStatusUpdate(it) }
        }
    }

    class SoundModeUpdate(val inner: SoundModeUpdatePacket) : Packet()
    class StateUpdate(val inner: StateUpdatePacket) : Packet()
    class SetSoundModeOk(val inner: SetSoundModeOkPacket) : Packet()
    class SetEqualizerOk(val inner: SetEqualizerOkPacket) : Packet()
    class SetEqualizerWithDrcOk(val inner: SetEqualizerWithDrcOkPacket) : Packet()
    class BatteryChargingUpdate(val inner: BatteryChargingUpdatePacket) : Packet()
    class BatteryLevelUpdate(val inner: BatteryLevelUpdatePacket) : Packet()
    class ChineseVoicePromptStateUpdate(val inner: ChineseVoicePromptStateUpdatePacket) : Packet()
    class FirmwareVersionUpdate(val inner: FirmwareVersionUpdatePacket) : Packet()
    class LdacStateUpdate(val inner: LdacStateUpdatePacket) : Packet()
    class TwsStatusUpdate(val inner: TwsStatusUpdatePacket) : Packet()

}
