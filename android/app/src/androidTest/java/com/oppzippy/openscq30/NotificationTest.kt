package com.oppzippy.openscq30

import android.content.Intent
import android.os.Build
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTextInput
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.rule.GrantPermissionRule
import androidx.test.uiautomator.By
import androidx.test.uiautomator.UiDevice
import androidx.test.uiautomator.UiObject2
import androidx.test.uiautomator.Until
import com.oppzippy.openscq30.actions.addAndConnectToDemoDevice
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.ui.OpenSCQ30Root
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.junit4.MockKRule
import java.util.regex.Pattern
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Before
import org.junit.Rule
import org.junit.Test

@HiltAndroidTest
class NotificationTest {
    @get:Rule(order = 0)
    val permissionRule: GrantPermissionRule = GrantPermissionRule.grant(
        android.Manifest.permission.POST_NOTIFICATIONS,
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            android.Manifest.permission.BLUETOOTH_CONNECT
        } else {
            android.Manifest.permission.BLUETOOTH
        },
    )

    @get:Rule(order = 1)
    val mockkRule = MockKRule(this)

    @get:Rule(order = 2)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 3)
    val composeRule = createAndroidComposeRule<TestActivity>()

    private lateinit var uiDevice: UiDevice

    private val notificationTitle = By.text("Connected to Soundcore Life Q30")
    private val notification: UiObject2
        get() {
            return uiDevice.findObject(notificationTitle).parent.parent.parent!!
        }

    private fun getString(id: Int): String = composeRule.activity.getString(id)
    private fun getString(id: Int, vararg formatArgs: Any): String = composeRule.activity.getString(id, *formatArgs)

    private fun expandNotification() {
        notification.findObject(By.desc("Expand"))?.click()
        val disconnect = By.desc(getString(R.string.disconnect))
        notification.wait(Until.hasObject(disconnect), 5000)
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
        composeRule.setContent {
            OpenSCQ30Root()
        }
        addAndConnectToDemoDevice(composeRule, "Soundcore Life Q30")

        uiDevice.pressHome()
        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        uiDevice.findObject(notificationTitle).click()
        uiDevice.wait(Until.hasObject(By.pkg("com.oppzippy.openscq30.debug")), 1000)
        assertEquals("com.oppzippy.openscq30.debug", uiDevice.currentPackageName)
    }

    @Test
    fun disconnectsAndClosesNotificationWhenDisconnectIsClicked() {
        composeRule.setContent {
            OpenSCQ30Root()
        }
        addAndConnectToDemoDevice(composeRule, "Soundcore Life Q30")

        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 1000)
        expandNotification()

        val disconnect = By.desc(getString(R.string.disconnect))
        notification.findObject(disconnect).click()

        uiDevice.wait(Until.gone(notificationTitle), 1000)
        assertFalse(uiDevice.hasObject(notificationTitle))
        uiDevice.pressBack()

        composeRule.onNodeWithContentDescription("Refresh").assertExists()
    }

    @Test
    fun quickPresetButtonsWork() {
        composeRule.setContent {
            OpenSCQ30Root()
        }
        addAndConnectToDemoDevice(composeRule, "Soundcore Life Q30")
        // Create the quick preset
        composeRule.onNodeWithText(getString(R.string.quick_presets)).performClick()
        composeRule.onNodeWithContentDescription(getString(R.string.create)).performClick()
        composeRule.onNodeWithText(getString(R.string.name)).performTextInput("My Preset 1")
        composeRule.onNodeWithText(getString(R.string.create)).performClick()
        composeRule.onNodeWithText(getString(R.string.edit)).performClick()
        composeRule.onNodeWithText("Ambient Sound Mode").performClick()
        composeRule.onNodeWithContentDescription(getString(R.string.back)).performClick()
        composeRule.onNodeWithContentDescription(getString(R.string.back)).performClick()
        // Add it to slot 1
        composeRule.onNodeWithText(getString(R.string.status_notification)).performClick()
        composeRule.onNodeWithTag(getString(R.string.quick_preset_slot_x, 1) + " select").performClick()
        composeRule.onNodeWithText("My Preset 1").performClick()
        composeRule.onNodeWithContentDescription(getString(R.string.back)).performClick()
        // Change the ambient sound mode away from the quick preset
        composeRule.onNodeWithText("Sound Modes").performClick()
        composeRule.onNodeWithText("Noise Canceling").performClick()
        composeRule.onNodeWithText("Normal").performClick()
        composeRule.onNodeWithText("Noise Canceling").assertDoesNotExist()

        // Activate the quick preset via the notification
        uiDevice.openNotification()
        uiDevice.wait(Until.hasObject(notificationTitle), 5000)
        expandNotification()
        notification.findObject(By.text(Pattern.compile("My Preset 1", Pattern.CASE_INSENSITIVE))).click()
        uiDevice.pressBack()

        composeRule.onNodeWithText("Noise Canceling")
            .assertExists("ambient sound mode should change when activating quick preset")
    }
}
