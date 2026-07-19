package com.oppzippy.openscq30.features.callautomation

import android.media.AudioManager
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class CallTransparencyControllerTest {
    @Test
    fun `switches to transparency during a call and restores the previous mode`() = runTest {
        val gateway = FakeAmbientModeGateway("NoiseCanceling")
        val controller = CallTransparencyController(gateway)

        controller.onCallStateChanged(true)
        controller.onCallStateChanged(false)

        assertEquals(listOf("Transparency", "NoiseCanceling"), gateway.setModes)
        assertEquals("NoiseCanceling", gateway.currentMode())
    }

    @Test
    fun `does not change or restore when transparency was already active`() = runTest {
        val gateway = FakeAmbientModeGateway("Transparency")
        val controller = CallTransparencyController(gateway)

        controller.onCallStateChanged(true)
        controller.onCallStateChanged(false)

        assertTrue(gateway.setModes.isEmpty())
    }

    @Test
    fun `does not overwrite a mode selected by the user during the call`() = runTest {
        val gateway = FakeAmbientModeGateway("NoiseCanceling")
        val controller = CallTransparencyController(gateway)

        controller.onCallStateChanged(true)
        gateway.mode = "Normal"
        controller.onCallStateChanged(false)

        assertEquals(listOf("Transparency"), gateway.setModes)
        assertEquals("Normal", gateway.currentMode())
    }

    @Test
    fun `ignores repeated call state notifications`() = runTest {
        val gateway = FakeAmbientModeGateway("Normal")
        val controller = CallTransparencyController(gateway)

        controller.onCallStateChanged(true)
        controller.onCallStateChanged(true)
        controller.onCallStateChanged(false)
        controller.onCallStateChanged(false)

        assertEquals(listOf("Transparency", "Normal"), gateway.setModes)
    }

    @Test
    fun `recognizes phone and internet call audio modes`() {
        assertTrue(isCallAudioMode(AudioManager.MODE_IN_CALL))
        assertTrue(isCallAudioMode(AudioManager.MODE_IN_COMMUNICATION))
        assertFalse(isCallAudioMode(AudioManager.MODE_NORMAL))
        assertFalse(isCallAudioMode(AudioManager.MODE_RINGTONE))
    }

    private class FakeAmbientModeGateway(initialMode: String?) : AmbientModeGateway {
        var mode = initialMode
        val setModes = mutableListOf<String>()

        override fun currentMode(): String? = mode

        override suspend fun setMode(mode: String) {
            this.mode = mode
            setModes += mode
        }
    }
}
