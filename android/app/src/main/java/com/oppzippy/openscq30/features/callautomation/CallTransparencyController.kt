package com.oppzippy.openscq30.features.callautomation

import com.oppzippy.openscq30.lib.bindings.OpenScq30Device
import com.oppzippy.openscq30.lib.bindings.SettingIdValuePair
import com.oppzippy.openscq30.lib.wrapper.Setting
import com.oppzippy.openscq30.lib.wrapper.toValue

internal const val AMBIENT_SOUND_MODE_SETTING_ID = "ambientSoundMode"
internal const val TRANSPARENCY_MODE = "Transparency"

internal interface AmbientModeGateway {
    fun currentMode(): String?

    suspend fun setMode(mode: String)
}

internal class OpenScq30AmbientModeGateway(private val device: OpenScq30Device) : AmbientModeGateway {
    override fun currentMode(): String? =
        (device.setting(AMBIENT_SOUND_MODE_SETTING_ID) as? Setting.SelectSetting)?.value

    override suspend fun setMode(mode: String) {
        device.setSettingValues(
            listOf(SettingIdValuePair(AMBIENT_SOUND_MODE_SETTING_ID, mode.toValue())),
        )
    }
}

internal class CallTransparencyController(private val ambientModeGateway: AmbientModeGateway) {
    private var callActive = false
    private var previousMode: String? = null
    private var changedByAutomation = false

    suspend fun onCallStateChanged(active: Boolean) {
        if (active == callActive) {
            return
        }

        if (active) {
            activateTransparency()
        } else {
            restorePreviousMode()
        }
    }

    private suspend fun activateTransparency() {
        val currentMode = ambientModeGateway.currentMode() ?: return
        if (currentMode != TRANSPARENCY_MODE) {
            ambientModeGateway.setMode(TRANSPARENCY_MODE)
            previousMode = currentMode
            changedByAutomation = true
        }
        callActive = true
    }

    private suspend fun restorePreviousMode() {
        val modeToRestore = previousMode?.takeIf {
            changedByAutomation && ambientModeGateway.currentMode() == TRANSPARENCY_MODE
        }
        callActive = false
        previousMode = null
        changedByAutomation = false
        if (modeToRestore != null) {
            ambientModeGateway.setMode(modeToRestore)
        }
    }
}
