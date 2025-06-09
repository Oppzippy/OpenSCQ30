package com.oppzippy.openscq30

import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import com.oppzippy.openscq30.actions.addAndConnectToDemoDevice
import com.oppzippy.openscq30.features.equalizer.storage.LegacyEqualizerProfile
import com.oppzippy.openscq30.features.equalizer.storage.LegacyEqualizerProfileDao
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import dagger.hilt.android.testing.HiltAndroidTest
import javax.inject.Inject
import kotlinx.coroutines.runBlocking
import org.junit.Test

@HiltAndroidTest
class MigrateLegacyEqualizerProfilesTest : OpenSCQ30RootTestBase() {
    @Inject
    lateinit var legacyEqualizerProfileDao: LegacyEqualizerProfileDao

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
