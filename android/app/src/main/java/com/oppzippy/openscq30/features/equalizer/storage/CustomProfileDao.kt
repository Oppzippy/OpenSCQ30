package com.oppzippy.openscq30.features.equalizer.storage

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import kotlinx.coroutines.flow.Flow

@Dao
interface CustomProfileDao {
    @Query("SELECT * FROM custom_equalizer_profile")
    suspend fun getAll(): List<CustomProfile>

    @Query("SELECT * FROM custom_equalizer_profile")
    fun all(): Flow<List<CustomProfile>>

    @Query("SELECT * FROM custom_equalizer_profile WHERE name = :name")
    suspend fun get(name: String): CustomProfile?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(customProfile: CustomProfile)

    @Query("DELETE FROM custom_equalizer_profile WHERE name = :name")
    suspend fun delete(name: String)
}
