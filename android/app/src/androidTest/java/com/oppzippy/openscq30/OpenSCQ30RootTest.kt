package com.oppzippy.openscq30

import android.content.Intent
import android.os.Build
import androidx.compose.ui.test.ExperimentalTestApi
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.lifecycle.lifecycleScope
import androidx.test.rule.GrantPermissionRule
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import com.oppzippy.openscq30.features.soundcoredevice.impl.DemoSoundcoreDeviceConnector
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.ui.OpenSCQ30Root
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.coEvery
import io.mockk.every
import io.mockk.junit4.MockKRule
import javax.inject.Inject
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Before
import org.junit.Rule
import org.junit.Test

@OptIn(ExperimentalTestApi::class)
@HiltAndroidTest
class OpenSCQ30RootTest {
    @get:Rule
    val permissionRule: GrantPermissionRule = GrantPermissionRule.grant(
        android.Manifest.permission.POST_NOTIFICATIONS,
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            android.Manifest.permission.BLUETOOTH_CONNECT
        } else {
            android.Manifest.permission.BLUETOOTH
        },
    )

    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<TestActivity>()

    @Inject
    lateinit var bluetoothDeviceProvider: BluetoothDeviceProvider

    @Inject
    lateinit var soundcoreDeviceConnector: SoundcoreDeviceConnector

    @Before
    fun setUp() {
        hiltRule.inject()
    }

    @After
    fun tearDown() {
        composeRule.activity.stopService(Intent(composeRule.activity, DeviceService::class.java))
    }

    @Test
    fun itWorks() = runTest {
        val bluetoothDevices = listOf(BluetoothDevice("Demo Device", "00:00:00:00:00:00"))
        every { bluetoothDeviceProvider.getDevices() } returns bluetoothDevices
        composeRule.setContent {
            OpenSCQ30Root()
        }

        val device = DemoSoundcoreDeviceConnector().connectToSoundcoreDevice(
            macAddress = "00:00:00:00:00:00",
            coroutineScope = composeRule.activity.lifecycleScope,
        )
        coEvery { soundcoreDeviceConnector.connectToSoundcoreDevice(any(), any()) } returns device

        composeRule.onNodeWithText("Demo Device").performClick()
        val general = hasTextExactly(composeRule.activity.getString(R.string.general))
        composeRule.waitUntilNodeCount(general, 1, 1500)
        composeRule.onNode(general).assertExists()
    }
}
