package com.oppzippy.openscq30

import android.content.ClipboardManager
import androidx.compose.ui.test.assertTextContains
import androidx.compose.ui.test.assertTextEquals
import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.compose.ui.test.onNodeWithTag
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTextInput
import androidx.core.content.getSystemService
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.ui.importexport.ImportExportScreen
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.junit4.MockKRule
import kotlinx.coroutines.test.runTest
import org.junit.Assert
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import javax.inject.Inject

@HiltAndroidTest
class DeviceSettingsImportExportTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<TestActivity>()

    @Inject
    lateinit var customProfileDao: CustomProfileDao

    @Before
    fun setUp() {
        hiltRule.inject()
    }

    @Test
    fun testExport() = runTest {
        customProfileDao.insert(
            CustomProfile(
                "test profile", 1, 2, 3, 4, 5, 6, 7, 8,
            ),
        )
        composeRule.setContent {
            ImportExportScreen()
        }

        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.export_custom_profiles))
            .performClick()
        // Check the box for exporting the profile
        composeRule
            .onNodeWithText("test profile")
            .performClick()
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.next))
            .performClick()
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.copy_to_clipboard))
            .performClick()

        val manager = composeRule.activity.getSystemService<ClipboardManager>()!!
        val clipboardContent = manager.primaryClip!!.getItemAt(0)!!.text

        // make sure json display matches what is copied to clipboard
        composeRule
            .onNodeWithTag("json-output")
            .assertTextEquals(clipboardContent.toString())

        // TODO ensure output is correct
        composeRule
            .onNodeWithTag("json-output")
            .assertTextContains("test profile", substring = true)
    }

    @Test
    fun testImport() = runTest {
        composeRule.setContent {
            ImportExportScreen()
        }

        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.import_custom_profiles))
            .performClick()
        composeRule
            .onNodeWithTag("json-input")
            .performTextInput(
                """
                [
                  {
                    "name": "test profile",
                    "volumeAdjustments": [1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8]
                  }
                ]
                """.trimIndent(),
            )
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.next))
            .performClick()
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.import_))
            .performClick()
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.next))
            .performClick()

        val profiles = customProfileDao.getAll()
        Assert.assertEquals(
            profiles,
            listOf(CustomProfile("test profile", 11, 22, 33, 44, 55, 66, 77, 88)),
        )
    }

    @Test
    fun testImportDuplicateNoOverwrite() = runTest {
        customProfileDao.insert(
            CustomProfile(
                "test profile", 1, 2, 3, 4, 5, 6, 7, 8,
            ),
        )
        composeRule.setContent {
            ImportExportScreen()
        }

        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.import_custom_profiles))
            .performClick()
        composeRule
            .onNodeWithTag("json-input")
            .performTextInput(
                """
                [
                  {
                    "name": "test profile",
                    "volumeAdjustments": [1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8]
                  }
                ]
                """.trimIndent(),
            )
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.next))
            .performClick()
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.import_))
            .performClick()
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.next))
            .performClick()

        val profiles = customProfileDao.getAll()
        Assert.assertEquals(
            profiles.toSet(),
            setOf(
                CustomProfile("test profile", 1, 2, 3, 4, 5, 6, 7, 8),
                CustomProfile("test profile (2)", 11, 22, 33, 44, 55, 66, 77, 88),
            ),
        )
    }

    @Test
    fun testImportDuplicateOverwrite() = runTest {
        customProfileDao.insert(
            CustomProfile(
                "test profile", 1, 2, 3, 4, 5, 6, 7, 8,
            ),
        )
        composeRule.setContent {
            ImportExportScreen()
        }

        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.import_custom_profiles))
            .performClick()
        composeRule
            .onNodeWithTag("json-input")
            .performTextInput(
                """
                [
                  {
                    "name": "test profile",
                    "volumeAdjustments": [1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8]
                  }
                ]
                """.trimIndent(),
            )
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.next))
            .performClick()
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.overwrite))
            .performClick()
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.import_))
            .performClick()
        composeRule
            .onNodeWithText(composeRule.activity.getString(R.string.next))
            .performClick()

        val profiles = customProfileDao.getAll()
        Assert.assertEquals(
            profiles,
            listOf(
                CustomProfile("test profile", 11, 22, 33, 44, 55, 66, 77, 88),
            ),
        )
    }
}
