package com.oppzippy.openscq30

import androidx.compose.ui.test.assertIsOff
import androidx.compose.ui.test.assertIsOn
import androidx.compose.ui.test.assertTextContains
import androidx.compose.ui.test.click
import androidx.compose.ui.test.hasAnyAncestor
import androidx.compose.ui.test.hasText
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.isDialog
import androidx.compose.ui.test.isToggleable
import androidx.compose.ui.test.onAllNodesWithTag
import androidx.compose.ui.test.onFirst
import androidx.compose.ui.test.onNodeWithContentDescription
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTextClearance
import androidx.compose.ui.test.performTextInput
import androidx.compose.ui.test.performTouchInput
import androidx.compose.ui.test.swipeLeft
import com.oppzippy.openscq30.actions.addAndConnectToDemoDevice
import com.oppzippy.openscq30.extensions.assertRangeValueApproxEquals
import com.oppzippy.openscq30.lib.bindings.translateCategoryId
import com.oppzippy.openscq30.lib.bindings.translateDeviceModel
import com.oppzippy.openscq30.lib.bindings.translateSettingId
import dagger.hilt.android.testing.HiltAndroidTest
import org.junit.Test

@HiltAndroidTest
class SettingTest : OpenSCQ30RootTestBase() {
    @Test
    fun testToggle() {
        addAndConnectToDemoDevice(composeRule, translateDeviceModel("SoundcoreA3959"))
        composeRule.onNodeWithText(translateCategoryId("soundModes")).performClick()
        composeRule.onNodeWithText(translateSettingId("windNoiseSuppression"))
            .assertIsOff()
            .performClick()
            .assertIsOn()
    }

    @Test
    fun testI32Range() {
        addAndConnectToDemoDevice(composeRule, translateDeviceModel("SoundcoreA3951"))
        composeRule.onNodeWithText(translateCategoryId("soundModes")).performClick()
        composeRule.onNodeWithTag(translateSettingId("customNoiseCanceling") + " slider")
            .assertRangeValueApproxEquals(0f)
            .performTouchInput { click(centerRight) }
            .assertRangeValueApproxEquals(10f)
    }

    @Test
    fun testSelect() {
        addAndConnectToDemoDevice(composeRule, translateDeviceModel("SoundcoreA3951"))
        composeRule.onNodeWithText(translateCategoryId("soundModes")).performClick()
        composeRule.onNodeWithText("Noise Canceling").performClick()
        composeRule.onNodeWithText("Normal").performClick().assertExists()
        composeRule.onNodeWithText("Noise Canceling").assertDoesNotExist()
    }

    @Test
    fun testOptionalSelect() {
        addAndConnectToDemoDevice(composeRule, translateDeviceModel("SoundcoreA3951"))
        composeRule.onNodeWithText(translateCategoryId("equalizer")).performClick()
        composeRule.onNodeWithText("Soundcore Signature").performClick()
        composeRule.onNode(hasTextExactly(getString(R.string.none)) and hasAnyAncestor(isDialog()))
            .performClick()
        composeRule.onNode(hasTextExactly(getString(R.string.none)) and !hasAnyAncestor(isDialog()))
        composeRule.onNodeWithText("Soundcore Signature").assertDoesNotExist()
    }

    @Test
    fun testModifiableSelect() {
        addAndConnectToDemoDevice(composeRule, translateDeviceModel("SoundcoreA3951"))
        composeRule.onNodeWithText(translateCategoryId("equalizer")).performClick()

        // Add a custom equalizer profile
        composeRule.onNodeWithContentDescription(getString(R.string.add)).performClick()
        composeRule.onNodeWithText(getString(R.string.name)).performTextInput("Test Profile")
        composeRule.onNodeWithText(getString(R.string.create)).performClick()

        // Select it
        composeRule.onNodeWithText(getString(R.string.none)).performClick()
        composeRule.onNodeWithText("Test Profile").performClick().assertExists()

        // Delete it
        composeRule.onNodeWithContentDescription(getString(R.string.delete)).performClick()
        composeRule.onNodeWithText(getString(R.string.delete)).performClick()
        composeRule.onNodeWithText("Test Profile").assertDoesNotExist()
    }

    @Test
    fun testMultiSelect() {
        addAndConnectToDemoDevice(composeRule, translateDeviceModel("SoundcoreA3951"))

        // Add a custom equalizer profile
        // TODO add multiple profiles
        composeRule.onNodeWithText(translateCategoryId("equalizer")).performClick()
        composeRule.onNodeWithContentDescription(getString(R.string.add)).performClick()
        composeRule.onNodeWithText(getString(R.string.name)).performTextInput("Test Profile")
        composeRule.onNodeWithText(getString(R.string.create)).performClick()
        composeRule.onNodeWithContentDescription(getString(R.string.back)).performClick()

        // Select it for export
        composeRule.onNodeWithText(translateCategoryId("equalizerImportExport")).performClick()
        composeRule.onNodeWithText(getString(R.string.none)).performClick()
        composeRule.onNode(hasTextExactly("Test Profile") and isToggleable()).performClick()
        composeRule.waitForIdle()
        composeRule.onNode(hasTextExactly("Test Profile") and isToggleable()).assertIsOn()
    }

    @Test
    fun testEqualizer() {
        addAndConnectToDemoDevice(composeRule, translateDeviceModel("SoundcoreA3951"))
        composeRule.onNodeWithText(translateCategoryId("equalizer")).performClick()
        val firstBandTextInput = hasText(getString(R.string.hz, 100))
        composeRule.onNode(firstBandTextInput).performTextClearance()
        composeRule.onNode(firstBandTextInput).performTextInput("2")
        composeRule.onNode(firstBandTextInput).assertTextContains("2.0")
        composeRule.onAllNodesWithTag("equalizerSlider").onFirst().performTouchInput { swipeLeft(centerY, left) }
        composeRule.onNode(firstBandTextInput).assertTextContains("-12.0")
    }

    @Test
    fun testImportString() {
        addAndConnectToDemoDevice(composeRule, translateDeviceModel("SoundcoreA3951"))
        composeRule.onNodeWithText(translateCategoryId("equalizerImportExport")).performClick()
        composeRule.onNodeWithText(translateSettingId("importCustomEqualizerProfiles")).performTextInput(
            """[{"name": "test profile", "volumeAdjustments": [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]}]""",
        )
        composeRule.onNodeWithContentDescription(getString(R.string.import_)).performClick()
        composeRule.onNodeWithText(getString(R.string.confirm)).performClick()
        composeRule.onNodeWithText(getString(R.string.none)).performClick()
        composeRule.onNode(hasTextExactly("test profile") and hasAnyAncestor(isDialog())).assertExists()
    }
}
