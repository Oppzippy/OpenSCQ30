package com.oppzippy.openscq30

import androidx.compose.ui.test.junit4.createAndroidComposeRule
import androidx.sqlite.db.SimpleSQLiteQuery
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPreset
import com.oppzippy.openscq30.features.quickpresets.storage.QuickPresetRepository
import com.oppzippy.openscq30.room.AppDatabase
import dagger.hilt.android.testing.HiltAndroidRule
import dagger.hilt.android.testing.HiltAndroidTest
import io.mockk.junit4.MockKRule
import java.util.UUID
import javax.inject.Inject
import kotlinx.coroutines.test.runTest
import org.junit.Assert
import org.junit.Before
import org.junit.Rule
import org.junit.Test

@HiltAndroidTest
class QuickPresetsMigrationTest {
    @get:Rule
    val mockkRule = MockKRule(this)

    @get:Rule(order = 1)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 2)
    val composeRule = createAndroidComposeRule<TestActivity>()

    @Inject
    lateinit var quickPresetRepository: QuickPresetRepository

    @Inject
    lateinit var appDatabase: AppDatabase

    @Before
    fun setUp() {
        hiltRule.inject()
    }

    @Test
    fun itMigratesServiceUuidsToDeviceModels() = runTest {
        val uuid = UUID.randomUUID()
        val unmigratedUuid = UUID.randomUUID()
        quickPresetRepository.insert(QuickPreset(deviceModel = null, deviceBleServiceUuid = uuid, index = 0))
        quickPresetRepository.insert(QuickPreset(deviceModel = null, deviceBleServiceUuid = uuid, index = 1))
        quickPresetRepository.insert(QuickPreset(deviceModel = null, deviceBleServiceUuid = unmigratedUuid, index = 0))
        quickPresetRepository.migrateBleServiceUuids("0123", uuid)

        val migratedPresets = quickPresetRepository.getForDevice("0123")
        migratedPresets.forEach { Assert.assertNull(it.deviceBleServiceUuid) }
        migratedPresets.forEach { Assert.assertNotNull(it.deviceModel) }

        // Verify number of unmigrated rows
        val cursor =
            appDatabase.query(
                SimpleSQLiteQuery("SELECT COUNT(*) FROM quick_preset WHERE deviceBleServiceUuid IS NOT NULL"),
            )
        Assert.assertTrue(cursor.moveToNext())
        Assert.assertEquals(1, cursor.getInt(0))
    }
}
