package com.oppzippy.openscq30.features.quickpresets.storage

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import java.util.UUID
import kotlinx.coroutines.flow.Flow

@Dao
interface QuickPresetDao {
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertFallback(quickPreset: FallbackQuickPreset)

    @Query("SELECT * FROM fallback_quick_preset")
    suspend fun getFallbacks(): List<FallbackQuickPreset>

    @Query("SELECT * FROM fallback_quick_preset WHERE `index` = :index")
    suspend fun getFallback(index: Int): FallbackQuickPreset?

    @Query("SELECT `index`, name FROM fallback_quick_preset ORDER BY `index` ASC")
    fun allFallbackNames(): Flow<List<QuickPresetIndexAndName>>

    @Query("SELECT * FROM quick_preset WHERE deviceBleServiceUuid = :deviceBleServiceUuid")
    suspend fun getForDeviceByServiceUuid(deviceBleServiceUuid: UUID): List<QuickPreset>

    @Query("SELECT * FROM quick_preset WHERE deviceModel = :deviceModel")
    suspend fun getForDevice(deviceModel: String): List<QuickPreset>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(quickPreset: QuickPreset)

    @Query("DELETE FROM quick_preset WHERE deviceModel = :deviceModel AND `index` = :index")
    suspend fun delete(deviceModel: String, index: Int)

    @Query(
        "SELECT `index`, name FROM quick_preset WHERE deviceModel = :deviceModel ORDER BY `index` ASC",
    )
    fun allNames(deviceModel: String): Flow<List<QuickPresetIndexAndName>>
}

data class QuickPresetIndexAndName(val index: Int, val name: String?)
