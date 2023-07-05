package com.oppzippy.openscq30.features.quickpresets.storage

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query

@Dao
interface QuickPresetDao {
    @Query("SELECT * FROM quick_preset")
    suspend fun getAll(): List<QuickPreset>

    @Query("SELECT * FROM quick_preset WHERE id = :id")
    suspend fun get(id: Int): QuickPreset?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(customProfile: QuickPreset)

    @Query("DELETE FROM quick_preset WHERE id = :id")
    suspend fun delete(id: Int)
}
