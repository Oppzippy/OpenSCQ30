package com.oppzippy.openscq30.features.equalizer.storage

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import kotlinx.coroutines.flow.Flow

@Dao
interface CustomProfileDao {
    @Query("SELECT * FROM equalizer_custom_profile")
    suspend fun getAll(): List<CustomProfile>

    @Query("SELECT name FROM equalizer_custom_profile")
    fun allNames(): Flow<List<String>>

    @Query("SELECT * FROM equalizer_custom_profile WHERE name = :name")
    suspend fun get(name: String): CustomProfile?

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(customProfile: CustomProfile)

    @Query("DELETE FROM equalizer_custom_profile WHERE name = :name")
    suspend fun delete(name: String)
}
