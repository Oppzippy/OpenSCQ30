package com.oppzippy.openscq30.features.quickpresets.storage

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import kotlinx.coroutines.flow.Flow

@Dao
interface QuickPresetDao {
    @Query("SELECT * FROM quick_preset")
    suspend fun getAll(): List<QuickPreset>

    @Query("SELECT * FROM quick_preset WHERE id = :id")
    suspend fun get(id: Int): QuickPreset?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(quickPreset: QuickPreset)

    @Query("DELETE FROM quick_preset WHERE id = :id")
    suspend fun delete(id: Int)

    @Query("SELECT id, name FROM quick_preset ORDER BY id ASC")
    fun allNames(): Flow<List<QuickPresetIdAndName>>
}

data class QuickPresetIdAndName(val id: Int, val name: String?)
