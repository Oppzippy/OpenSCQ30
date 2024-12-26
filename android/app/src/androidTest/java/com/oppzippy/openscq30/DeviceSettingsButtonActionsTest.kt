package com.oppzippy.openscq30

import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import com.oppzippy.openscq30.lib.wrapper.ButtonAction
import com.oppzippy.openscq30.lib.wrapper.ButtonConfiguration
import com.oppzippy.openscq30.lib.wrapper.MultiButtonConfiguration
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
        var actions = MultiButtonConfiguration(
            leftSingleClick = ButtonConfiguration(true, ButtonAction.NextSong),
            leftDoubleClick = ButtonConfiguration(false, ButtonAction.NextSong),
            leftLongPress = ButtonConfiguration(false, ButtonAction.NextSong),
            rightSingleClick = ButtonConfiguration(false, ButtonAction.NextSong),
            rightDoubleClick = ButtonConfiguration(false, ButtonAction.NextSong),
            rightLongPress = ButtonConfiguration(false, ButtonAction.NextSong),
        )
        composeRule.setContent {
            ButtonActionSelection(
                buttonConfiguration = actions,
                onChange = { actions = it },
            )
        }

        composeRule.onNodeWithText(composeRule.activity.getString(ButtonAction.NextSong.toStringResource()))
            .performClick()
        composeRule.onNodeWithText(composeRule.activity.getString(ButtonAction.PreviousSong.toStringResource()))
            .performClick()
        Assert.assertEquals(
            MultiButtonConfiguration(
                leftSingleClick = ButtonConfiguration(true, ButtonAction.PreviousSong),
                leftDoubleClick = ButtonConfiguration(false, ButtonAction.NextSong),
                leftLongPress = ButtonConfiguration(false, ButtonAction.NextSong),
                rightSingleClick = ButtonConfiguration(false, ButtonAction.NextSong),
                rightDoubleClick = ButtonConfiguration(false, ButtonAction.NextSong),
                rightLongPress = ButtonConfiguration(false, ButtonAction.NextSong),
            ),
            actions,
        )
    }
}
