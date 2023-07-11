package com.oppzippy.openscq30

import android.content.Intent
import android.os.Build
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.rule.GrantPermissionRule
import androidx.test.uiautomator.By
import androidx.test.uiautomator.UiDevice
import androidx.test.uiautomator.UiObject2
import androidx.test.uiautomator.Until
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetDao
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceFactory
import com.oppzippy.openscq30.features.soundcoredevice.demo.DemoSoundcoreDevice
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.AmbientSoundMode
import com.oppzippy.openscq30.lib.PresetEqualizerProfile
import com.oppzippy.openscq30.ui.OpenSCQ30Root
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.coEvery
import io.mockk.every
import io.mockk.junit4.MockKRule
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.withContext
import kotlinx.coroutines.withTimeout
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import javax.inject.Inject
import kotlin.jvm.optionals.getOrNull
import kotlin.time.Duration.Companion.seconds

@OptIn(ExperimentalCoroutinesApi::class)
@HiltAndroidTest
class NotificationTest {
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

    @Inject
    lateinit var quickPresetDao: QuickPresetDao

    @Inject
    lateinit var customProfileDao: CustomProfileDao

    private lateinit var uiDevice: UiDevice

    private val notificationTitle = By.text("Connected to Test Device")
    private val notification: UiObject2
        get() {
            return uiDevice.findObject(notificationTitle).parent.parent.parent!!
        }

    private fun expandNotification() {
        notification.findObject(By.desc("Expand"))?.click()
        val disconnect = By.text(composeRule.activity.getString(R.string.disconnect))
        notification.wait(Until.hasObject(disconnect), 1000)
    }

    @Before
    fun setUp() {
        hiltRule.inject()
        uiDevice = UiDevice.getInstance(InstrumentationRegistry.getInstrumentation())
    }

    @After
    fun tearDown() {
        uiDevice.pressHome()
        composeRule.activity.stopService(Intent(composeRule.activity, DeviceService::class.java))
    }

    @Test
    fun opensAppWhenNotificationIsClicked() {
        setUpDevice()

        uiDevice.pressHome()
        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        uiDevice.findObject(notificationTitle).click()
        uiDevice.wait(Until.hasObject(By.pkg("com.oppzippy.openscq30.debug")), 1000)
        assertEquals("com.oppzippy.openscq30.debug", uiDevice.currentPackageName)
    }

    @Test
    fun disconnectsAndClosesNotificationWhenDisconnectIsClicked() {
        setUpDevice()

        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        expandNotification()

        val disconnect = By.text(composeRule.activity.getString(R.string.disconnect))
        notification.findObject(disconnect).click()

        uiDevice.wait(Until.gone(notificationTitle), 1000)
        assertFalse(uiDevice.hasObject(notificationTitle))
        uiDevice.pressBack()

        composeRule.onNodeWithContentDescription("Refresh").assertExists()
    }

    @Test
    fun quickPresetButtonsWork() = runTest {
        quickPresetDao.insert(QuickPreset(id = 0, ambientSoundMode = AmbientSoundMode.Transparency))
        quickPresetDao.insert(
            QuickPreset(
                id = 1,
                name = "Test Preset 2",
                ambientSoundMode = AmbientSoundMode.NoiseCanceling,
            ),
        )
        val device = setUpDevice()

        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        expandNotification()

        val quickPreset1 = By.text(composeRule.activity.getString(R.string.quick_preset_number, 1))
        uiDevice.wait(Until.hasObject(quickPreset1.clickable(true)), 1000)
        notification.findObject(quickPreset1).click()

        // The test dispatcher skips delays, but waiting is necessary for the click event to be handled.
        withContext(Dispatchers.Default) {
            withTimeout(1.seconds) {
                device.stateFlow.first { it.ambientSoundMode() == AmbientSoundMode.Transparency }
            }
        }

        val quickPreset2 = By.text("Test Preset 2")
        notification.findObject(quickPreset2).click()
        withContext(Dispatchers.Default) {
            withTimeout(1.seconds) {
                device.stateFlow.first { it.ambientSoundMode() == AmbientSoundMode.NoiseCanceling }
            }
        }
    }

    @Test
    fun quickPresetEqualizerConfigurationWorks() = runTest {
        customProfileDao.insert(CustomProfile("Test Profile", listOf(0, 1, 2, 3, 4, 5, 6, 7)))
        quickPresetDao.insert(QuickPreset(id = 0, customEqualizerProfileName = "Test Profile"))
        quickPresetDao.insert(
            QuickPreset(id = 1, presetEqualizerProfile = PresetEqualizerProfile.SoundcoreSignature),
        )
        val device = setUpDevice()

        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        expandNotification()

        // Custom equalizer profile
        val quickPreset1 = By.text(composeRule.activity.getString(R.string.quick_preset_number, 1))
        uiDevice.wait(Until.hasObject(quickPreset1.clickable(true)), 1000)
        notification.findObject(quickPreset1).click()

        // The test dispatcher skips delays, but waiting is necessary for the click event to be handled.
        withContext(Dispatchers.Default) {
            withTimeout(2.seconds) {
                device.stateFlow.first { it.equalizerConfiguration().presetProfile().isEmpty }
            }
        }

        // Preset equalizer profile
        val quickPreset2 = By.text(composeRule.activity.getString(R.string.quick_preset_number, 2))
        notification.findObject(quickPreset2).click()
        withContext(Dispatchers.Default) {
            withTimeout(2.seconds) {
                device.stateFlow.first {
                    it.equalizerConfiguration()
                        .presetProfile().getOrNull() == PresetEqualizerProfile.SoundcoreSignature
                }
            }
        }
    }

    private fun setUpDevice(): DemoSoundcoreDevice {
        val bluetoothDevices = listOf(BluetoothDevice("Test Device", "00:00:00:00:00:00"))
        every { bluetoothDeviceProvider.getDevices() } returns bluetoothDevices
        val device = DemoSoundcoreDevice("Test Device", "00:00:00:00:00:00")
        coEvery {
            soundcoreDeviceFactory.createSoundcoreDevice(
                "00:00:00:00:00:00",
                any(),
            )
        } coAnswers {
            device
        }

        composeRule.setContent {
            OpenSCQ30Root()
        }
        composeRule.onNodeWithText("Test Device").performClick()
        return device
    }
}
