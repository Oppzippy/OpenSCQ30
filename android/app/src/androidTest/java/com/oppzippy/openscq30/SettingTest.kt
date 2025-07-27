package com.oppzippy.openscq30

import androidx.compose.ui.test.assertIsOff
import androidx.compose.ui.test.assertIsOn
import androidx.compose.ui.test.click
import androidx.compose.ui.test.hasAnyAncestor
import androidx.compose.ui.test.hasTextExactly
import androidx.compose.ui.test.isDialog
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTouchInput
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
        composeRule.waitForIdle()
        composeRule.onNode(hasTextExactly("None") and hasAnyAncestor(isDialog()))
            .performClick()
        composeRule.onNode(hasTextExactly("None") and !hasAnyAncestor(isDialog()))
        composeRule.onNodeWithText("Soundcore Signature").assertDoesNotExist()
    }
}
