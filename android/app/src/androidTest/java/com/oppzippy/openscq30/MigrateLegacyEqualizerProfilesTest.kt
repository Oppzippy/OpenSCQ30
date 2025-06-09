package com.oppzippy.openscq30

import android.content.Intent
import android.os.Build
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.test.rule.GrantPermissionRule
import com.oppzippy.openscq30.actions.addAndConnectToDemoDevice
import com.oppzippy.openscq30.features.equalizer.storage.LegacyEqualizerProfile
import com.oppzippy.openscq30.features.equalizer.storage.LegacyEqualizerProfileDao
import com.oppzippy.openscq30.features.soundcoredevice.service.DeviceService
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import com.oppzippy.openscq30.ui.OpenSCQ30Root
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.junit4.MockKRule
import javax.inject.Inject
import kotlinx.coroutines.runBlocking
import org.junit.After
import org.junit.Before
import org.junit.Rule
import org.junit.Test

@HiltAndroidTest
class MigrateLegacyEqualizerProfilesTest {
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

    @Inject
    lateinit var legacyEqualizerProfileDao: LegacyEqualizerProfileDao

    private fun getString(id: Int): String = composeRule.activity.getString(id)
    private fun getString(id: Int, vararg formatArgs: Any): String = composeRule.activity.getString(id, *formatArgs)

    @Before
    fun setUp() {
        hiltRule.inject()
    }

    @After
    fun tearDown() {
        composeRule.activity.stopService(Intent(composeRule.activity, DeviceService::class.java))
    }

    @Test
    fun testMigrateLegacyProfile(): Unit = runBlocking {
        legacyEqualizerProfileDao.insert(
            LegacyEqualizerProfile(
                "Test Legacy Profile",
                10,
                -20,
                30,
                40,
                50,
                60,
                70,
                80,
            ),
        )
        composeRule.setContent {
            OpenSCQ30Root()
        }
        addAndConnectToDemoDevice(composeRule, "Soundcore Life Q30")

        composeRule.onNodeWithText(getString(R.string.migrate_legacy_equalizer_profiles)).performClick()
        composeRule.onNodeWithText(getString(R.string.migrate)).performClick()
        composeRule.onNodeWithContentDescription(getString(R.string.back)).performClick()
        composeRule.onNodeWithText(getString(R.string.equalizer)).performClick()
        composeRule.onNodeWithTag(translateSettingId("customEqualizerProfile") + " select").performClick()
        composeRule.onNodeWithText("Test Legacy Profile").performClick()
        composeRule.onNodeWithText("1.0").assertExists()
        composeRule.onNodeWithText("-2.0").assertExists()
    }
}
