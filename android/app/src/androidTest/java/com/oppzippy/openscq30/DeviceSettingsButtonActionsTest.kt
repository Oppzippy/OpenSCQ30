package com.oppzippy.openscq30

import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import com.oppzippy.openscq30.lib.wrapper.ButtonAction
import com.oppzippy.openscq30.lib.wrapper.ButtonState
import com.oppzippy.openscq30.lib.wrapper.CustomButtonActions
import com.oppzippy.openscq30.ui.buttonactions.ButtonActionSelection
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import org.junit.Assert
import org.junit.Before
import org.junit.Rule
import org.junit.Test

@HiltAndroidTest
class DeviceSettingsButtonActionsTest {
    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<TestActivity>()

    @Before
    fun setUp() {
        hiltRule.inject()
    }

    @Test
    fun itWorks() {
        var actions = CustomButtonActions(
            leftSingleClick = ButtonState(true, ButtonAction.NextSong),
            leftDoubleClick = ButtonState(false, ButtonAction.NextSong),
            leftLongPress = ButtonState(false, ButtonAction.NextSong),
            rightSingleClick = ButtonState(false, ButtonAction.NextSong),
            rightDoubleClick = ButtonState(false, ButtonAction.NextSong),
            rightLongPress = ButtonState(false, ButtonAction.NextSong),
        )
        composeRule.setContent {
            ButtonActionSelection(
                buttonActions = actions,
                onChange = { actions = it },
            )
        }

        composeRule.onNodeWithText(composeRule.activity.getString(ButtonAction.NextSong.toStringResource()))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(ButtonAction.PreviousSong.toStringResource()))
            .performClick()
        Assert.assertEquals(
            CustomButtonActions(
                leftSingleClick = ButtonState(true, ButtonAction.PreviousSong),
                leftDoubleClick = ButtonState(false, ButtonAction.NextSong),
                leftLongPress = ButtonState(false, ButtonAction.NextSong),
                rightSingleClick = ButtonState(false, ButtonAction.NextSong),
                rightDoubleClick = ButtonState(false, ButtonAction.NextSong),
                rightLongPress = ButtonState(false, ButtonAction.NextSong),
            ),
            actions,
        )
    }
}
