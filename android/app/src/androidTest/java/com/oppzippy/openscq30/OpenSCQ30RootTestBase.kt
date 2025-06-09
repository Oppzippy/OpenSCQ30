package com.oppzippy.openscq30

import android.content.Intent
import android.os.Build
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.test.rule.GrantPermissionRule
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import dagger.hilt.android.testing.HiltAndroidRule
import io.mockk.junit4.MockKRule
import org.junit.After
import org.junit.Before
import org.junit.Rule

@Suppress("LeakingThis")
open class OpenSCQ30RootTestBase {
    @get:Rule(order = 0)
    val bluetoothPermissionRule: GrantPermissionRule = GrantPermissionRule.grant(
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
    val composeRule = createAndroidComposeRule<MainActivity>()

    @Before
    fun setUpHiltRule() {
        hiltRule.inject()
    }

    @After
    fun tearDownService() {
        composeRule.activity.stopService(Intent(composeRule.activity, DeviceService::class.java))
    }

    fun getString(id: Int): String = composeRule.activity.getString(id)
    fun getString(id: Int, vararg formatArgs: Any): String = composeRule.activity.getString(id, *formatArgs)
}
