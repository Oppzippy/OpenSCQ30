package com.oppzippy.openscq30

import android.content.Context
import android.content.Intent
import android.os.Build
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.compose.ui.test.junit4.AndroidComposeTestRule
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.test.ext.junit.rules.ActivityScenarioRule
import androidx.test.rule.GrantPermissionRule
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import dagger.hilt.android.testing.HiltAndroidRule
import io.mockk.MockKAnnotations
import io.mockk.clearAllMocks
import io.mockk.every
import io.mockk.mockkStatic
import kotlin.reflect.KFunction3
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

    @Before
    fun baseSetUp() {
        MockKAnnotations.init(this)
        hiltRule.inject()
    }

    @After
    fun baseTearDown() {
        composeRule.activity.stopService(Intent(composeRule.activity, DeviceService::class.java))
        clearAllMocks()
    }

    fun getString(id: Int): String = composeRule.activity.getString(id)
    fun getString(id: Int, vararg formatArgs: Any): String = composeRule.activity.getString(id, *formatArgs)

    fun mockMakeToast() {
        val text: KFunction3<Context, CharSequence, Int, Toast> = Toast::makeText
        val resId: KFunction3<Context, Int, Int, Toast> = Toast::makeText
        mockkStatic(text)
        mockkStatic(resId)

        every { text(any(), any(), any()) } answers { callOriginal() }
        every { resId(any(), any(), any()) } answers { callOriginal() }
    }
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
