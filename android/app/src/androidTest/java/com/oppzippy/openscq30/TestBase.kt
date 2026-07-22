package com.oppzippy.openscq30

import android.content.Intent
import android.os.Build
import android.view.accessibility.AccessibilityEvent
import androidx.activity.ComponentActivity
import androidx.compose.ui.test.junit4.AndroidComposeTestRule
import androidx.compose.ui.test.junit4.v2.createAndroidComposeRule
import androidx.test.ext.junit.rules.ActivityScenarioRule
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.rule.GrantPermissionRule
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import dagger.hilt.android.testing.HiltAndroidRule
import org.junit.After
import org.junit.Before
import org.junit.Rule

@Suppress("LeakingThis")
open class TestBase<A : ComponentActivity>(
    private val composeRule: AndroidComposeTestRule<ActivityScenarioRule<A>, A>,
) {
    @get:Rule(order = 0)
    val bluetoothPermissionRule: GrantPermissionRule = GrantPermissionRule.grant(
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            android.Manifest.permission.BLUETOOTH_CONNECT
        } else {
            android.Manifest.permission.BLUETOOTH
        },
    )

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    val toasts = mutableListOf<String>()

    @Before
    fun baseSetUp() {
        hiltRule.inject()

        InstrumentationRegistry.getInstrumentation().uiAutomation.setOnAccessibilityEventListener { event ->
            if (event.eventType == AccessibilityEvent.TYPE_NOTIFICATION_STATE_CHANGED) {
                if (event.className == "android.widget.Toast") {
                    toasts.add(event.text.first().toString())
                }
            }
        }
    }

    @After
    fun baseTearDown() {
        composeRule.activity.stopService(Intent(composeRule.activity, DeviceService::class.java))
        InstrumentationRegistry.getInstrumentation().uiAutomation.setOnAccessibilityEventListener(null)
    }

    fun getString(id: Int): String = composeRule.activity.getString(id)
    fun getString(id: Int, vararg formatArgs: Any): String = composeRule.activity.getString(id, *formatArgs)
}

@Suppress("LeakingThis")
open class OpenSCQ30RootTestBase(
    @get:Rule(order = 3)
    val composeRule: AndroidComposeTestRule<ActivityScenarioRule<MainActivity>, MainActivity> =
        createAndroidComposeRule<MainActivity>(),
) : TestBase<MainActivity>(composeRule)

@Suppress("LeakingThis")
open class EmptyActivityTestBase(
    @get:Rule(order = 3)
    val composeRule: AndroidComposeTestRule<ActivityScenarioRule<TestActivity>, TestActivity> =
        createAndroidComposeRule<TestActivity>(),
) : TestBase<TestActivity>(composeRule)
