package com.oppzippy.openscq30

import android.widget.Toast
import com.oppzippy.openscq30.actions.addAndConnectToDemoDevice
import com.oppzippy.openscq30.actions.addDemoDevice
import com.oppzippy.openscq30.lib.bindings.OpenScq30Exception
import com.oppzippy.openscq30.lib.bindings.OpenScq30Session
import com.oppzippy.openscq30.lib.bindings.newSessionWithInMemoryDb
import com.oppzippy.openscq30.lib.hilt.OpenSCQ30SessionModule
import dagger.hilt.android.testing.BindValue
import dagger.hilt.android.testing.HiltAndroidTest
import dagger.hilt.android.testing.UninstallModules
import io.mockk.coEvery
import io.mockk.mockk
import io.mockk.spyk
import io.mockk.verify
import kotlinx.coroutines.runBlocking
import org.junit.Test

@UninstallModules(OpenSCQ30SessionModule::class)
@HiltAndroidTest
class ErrorHandlingTest : OpenSCQ30RootTestBase() {
    @BindValue
    val session: OpenScq30Session = spyk(runBlocking { newSessionWithInMemoryDb() })

    @Test
    fun showsToastWhenErrorPairing() {
        coEvery { session.pair(any()) } throws mockk<OpenScq30Exception>()
        mockMakeToast()

        addDemoDevice(composeRule, "Soundcore Life Q30")

        verify(exactly = 1) { Toast.makeText(any(), getString(R.string.error_pairing), any()) }
    }

    @Test
    fun showsToastWhenErrorConnecting() {
        coEvery { session.connectWithBackends(any(), any()) } throws mockk<OpenScq30Exception>()
        mockMakeToast()

        addAndConnectToDemoDevice(composeRule, "Soundcore Life Q30")

        verify(exactly = 1) { Toast.makeText(any(), getString(R.string.error_connecting), any()) }
    }
}
