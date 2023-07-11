package com.oppzippy.openscq30

import android.content.Intent
import android.os.Build
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.test.rule.GrantPermissionRule
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import com.oppzippy.openscq30.features.soundcoredevice.demo.DemoSoundcoreDevice
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.ui.OpenSCQ30Root
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.coEvery
import io.mockk.every
import io.mockk.junit4.MockKRule
import kotlinx.coroutines.sync.Mutex
import org.junit.After
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import javax.inject.Inject

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
    lateinit var soundcoreDeviceFactory: SoundcoreDeviceFactory

    @Before
    fun setUp() {
        hiltRule.inject()
    }

    @After
    fun tearDown() {
        composeRule.activity.stopService(Intent(composeRule.activity, DeviceService::class.java))
    }

    @Test
    fun itWorks() {
        val bluetoothDevices = listOf(BluetoothDevice("Test Device", "00:00:00:00:00:00"))
        every { bluetoothDeviceProvider.getDevices() } returns bluetoothDevices
        composeRule.setContent {
            OpenSCQ30Root()
        }

        val device = DemoSoundcoreDevice("test", "00:00:00:00:00:00")
        val mutex = Mutex(locked = true)
        coEvery {
            soundcoreDeviceFactory.createSoundcoreDevice(
                "00:00:00:00:00:00",
                any(),
            )
        } coAnswers {
            mutex.lock()
            device
        }
        composeRule.onNodeWithText("Test Device").performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.loading)).assertExists()
        mutex.unlock()
        composeRule.onNodeWithText(composeRule.activity.getString(R.string.ambient_sound_mode))
            .assertExists()
    }
}
