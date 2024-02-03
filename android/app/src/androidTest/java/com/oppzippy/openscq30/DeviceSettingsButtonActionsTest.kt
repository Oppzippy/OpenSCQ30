package com.oppzippy.openscq30

import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import com.oppzippy.openscq30.lib.wrapper.ButtonAction
import com.oppzippy.openscq30.ui.buttonactions.ButtonActionSelection
import com.oppzippy.openscq30.ui.buttonactions.ButtonActions
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
        var actions = ButtonActions(
            leftSingleClick = ButtonAction.NextSong,
            leftDoubleClick = null,
            leftLongPress = null,
            rightSingleClick = null,
            rightDoubleClick = null,
            rightLongPress = null,
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
            ButtonActions(
                leftSingleClick = ButtonAction.PreviousSong,
                leftDoubleClick = null,
                leftLongPress = null,
                rightSingleClick = null,
                rightDoubleClick = null,
                rightLongPress = null,
            ),
            actions,
        )
    }
}
