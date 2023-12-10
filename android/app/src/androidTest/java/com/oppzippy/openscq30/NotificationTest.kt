package com.oppzippy.openscq30

import android.content.Intent
import android.os.Build
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.lifecycle.lifecycleScope
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.rule.GrantPermissionRule
import androidx.test.uiautomator.By
import androidx.test.uiautomator.UiDevice
import androidx.test.uiautomator.UiObject2
import androidx.test.uiautomator.Until
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDevice
import com.oppzippy.openscq30.features.bluetoothdeviceprovider.BluetoothDeviceProvider
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.features.equalizer.storage.toCustomProfile
import com.oppzippy.openscq30.features.quickpresets.storage.FallbackQuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetRepository
import com.oppzippy.openscq30.features.soundcoredevice.api.SoundcoreDeviceConnector
import com.oppzippy.openscq30.features.soundcoredevice.impl.DemoSoundcoreDeviceConnector
import com.oppzippy.openscq30.features.soundcoredevice.impl.SoundcoreDevice
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.wrapper.AmbientSoundMode
import com.oppzippy.openscq30.lib.wrapper.PresetEqualizerProfile
import com.oppzippy.openscq30.ui.OpenSCQ30Root
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.coEvery
import io.mockk.every
import io.mockk.junit4.MockKRule
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.withTimeout
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import java.util.UUID
import javax.inject.Inject
import kotlin.time.Duration.Companion.seconds

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
    lateinit var soundcoreDeviceConnector: SoundcoreDeviceConnector

    @Inject
    lateinit var quickPresetRepository: QuickPresetRepository

    @Inject
    lateinit var customProfileDao: CustomProfileDao

    private lateinit var uiDevice: UiDevice

    private val deviceUuid = UUID(0, 0)
    private val notificationTitle = By.text("Connected to Demo Device")
    private val notification: UiObject2
        get() {
            return uiDevice.findObject(notificationTitle).parent.parent.parent!!
        }

    private fun expandNotification() {
        notification.findObject(By.desc("Expand"))?.click()
        val disconnect = By.desc(composeRule.activity.getString(R.string.disconnect))
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
    fun opensAppWhenNotificationIsClicked() = runTest {
        setUpDevice()

        uiDevice.pressHome()
        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        uiDevice.findObject(notificationTitle).click()
        uiDevice.wait(Until.hasObject(By.pkg("com.oppzippy.openscq30.debug")), 1000)
        assertEquals("com.oppzippy.openscq30.debug", uiDevice.currentPackageName)
    }

    @Test
    fun disconnectsAndClosesNotificationWhenDisconnectIsClicked() = runTest {
        setUpDevice()

        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        expandNotification()

        val disconnect = By.desc(composeRule.activity.getString(R.string.disconnect))
        notification.findObject(disconnect).click()

        uiDevice.wait(Until.gone(notificationTitle), 1000)
        assertFalse(uiDevice.hasObject(notificationTitle))
        uiDevice.pressBack()

        composeRule.onNodeWithContentDescription("Refresh").assertExists()
    }

    @Test
    fun quickPresetButtonsWork(): Unit = runBlocking {
        quickPresetRepository.insert(
            QuickPreset(
                deviceBleServiceUuid = deviceUuid,
                index = 0,
                ambientSoundMode = AmbientSoundMode.Transparency,
            ),
        )
        quickPresetRepository.insert(
            QuickPreset(
                deviceBleServiceUuid = deviceUuid,
                index = 1,
                name = "Test Preset 2",
                ambientSoundMode = AmbientSoundMode.NoiseCanceling,
            ),
        )
        val device = setUpDevice()

        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        expandNotification()

        val quickPreset1 =
            By.desc(composeRule.activity.getString(R.string.quick_preset_number, 1))
        uiDevice.wait(Until.hasObject(quickPreset1.clickable(true)), 1000)
        notification.findObject(quickPreset1).click()

        withTimeout(1.seconds) {
            device.stateFlow.first { it.soundModes?.ambientSoundMode == AmbientSoundMode.Transparency }
        }

        val quickPreset2 = By.desc("Test Preset 2")
        notification.findObject(quickPreset2).click()
        withTimeout(1.seconds) {
            device.stateFlow.first { it.soundModes?.ambientSoundMode == AmbientSoundMode.NoiseCanceling }
        }
    }

    @Test
    fun quickPresetEqualizerConfigurationWorks(): Unit = runBlocking {
        customProfileDao.insert(
            listOf(
                0.0,
                0.1,
                0.2,
                0.3,
                0.4,
                0.5,
                0.6,
                0.7,
            ).toCustomProfile("Test Profile"),
        )
        quickPresetRepository.insert(
            QuickPreset(
                deviceBleServiceUuid = deviceUuid,
                index = 0,
                customEqualizerProfileName = "Test Profile",
            ),
        )
        quickPresetRepository.insert(
            QuickPreset(
                deviceBleServiceUuid = deviceUuid,
                index = 1,
                presetEqualizerProfile = PresetEqualizerProfile.SoundcoreSignature,
            ),
        )
        val device = setUpDevice()

        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        expandNotification()

        // Custom equalizer profile
        val quickPreset1 = By.desc(composeRule.activity.getString(R.string.quick_preset_number, 1))
        uiDevice.wait(Until.hasObject(quickPreset1.clickable(true)), 1000)
        notification.findObject(quickPreset1).click()

        withTimeout(2.seconds) {
            device.stateFlow.first { it.equalizerConfiguration.presetProfile == null }
        }

        // Preset equalizer profile
        val quickPreset2 = By.desc(composeRule.activity.getString(R.string.quick_preset_number, 2))
        notification.findObject(quickPreset2).click()
        withTimeout(2.seconds) {
            device.stateFlow.first {
                it.equalizerConfiguration.presetProfile == PresetEqualizerProfile.SoundcoreSignature
            }
        }
    }

    @Test
    fun deviceSpecificProfileNamesOverrideFallbacks() = runTest {
        quickPresetRepository.insert(
            QuickPreset(
                deviceUuid,
                0,
                "device specific 1",
            ),
        )
        quickPresetRepository.insertFallback(
            FallbackQuickPreset(
                0,
                "fallback 1",
            ),
        )
        quickPresetRepository.insertFallback(
            FallbackQuickPreset(
                1,
                "fallback 2",
            ),
        )

        setUpDevice()

        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        expandNotification()

        assertNotNull("device specific 1", notification.findObject(By.desc("device specific 1")))
        assertNotNull("fallback 2", notification.findObject(By.desc("fallback 2")))
    }

    private suspend fun setUpDevice(): SoundcoreDevice {
        val bluetoothDevices = listOf(BluetoothDevice("Demo Device", "00:00:00:00:00:00"))
        every { bluetoothDeviceProvider.getDevices() } returns bluetoothDevices

        val device = DemoSoundcoreDeviceConnector().connectToSoundcoreDevice(
            macAddress = "00:00:00:00:00:00",
            coroutineScope = composeRule.activity.lifecycleScope,
        )
        coEvery {
            soundcoreDeviceConnector.connectToSoundcoreDevice(
                "00:00:00:00:00:00",
                any(),
            )
        } coAnswers {
            device
        }

        composeRule.setContent {
            OpenSCQ30Root()
        }
        composeRule.onNodeWithText("Demo Device").performClick()
        return device
    }
}
