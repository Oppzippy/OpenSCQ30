package com.oppzippy.openscq30.ui.equalizer.storage

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query

@Dao
interface CustomProfileDao {
    @Query("SELECT * FROM equalizer_custom_profile")
    suspend fun getAll(): List<CustomProfile>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(customProfile: CustomProfile)

    @Query("DELETE FROM equalizer_custom_profile WHERE name = :name")
    suspend fun delete(name: String)
}
