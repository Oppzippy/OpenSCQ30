package com.oppzippy.openscq30.features.equalizer.storage

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import androidx.room.Transaction
import kotlinx.coroutines.flow.Flow

@Dao
abstract class CustomProfileDao {
    @Query("SELECT * FROM custom_equalizer_profile")
    abstract suspend fun getAll(): List<CustomProfile>

    @Query("SELECT * FROM custom_equalizer_profile")
    abstract fun all(): Flow<List<CustomProfile>>

    @Query("SELECT * FROM custom_equalizer_profile WHERE name = :name")
    abstract suspend fun get(name: String): CustomProfile?

    @Insert(onConflict = OnConflictStrategy.ABORT)
    abstract suspend fun insert(customProfile: CustomProfile)

    @Insert(onConflict = OnConflictStrategy.IGNORE)
    abstract suspend fun insertIgnoreConflicts(customProfile: CustomProfile)

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    abstract suspend fun upsert(customProfile: CustomProfile)

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    abstract suspend fun upsertAll(customProfile: List<CustomProfile>)

    @Query("DELETE FROM custom_equalizer_profile WHERE name = :name")
    abstract suspend fun delete(name: String)

    @Transaction
    open suspend fun insertAndRename(profiles: List<CustomProfile>) {
        profiles.forEach { profile ->
            val name = findRename(profile.name)
            // skip if we can't find a suitable name
            if (name != null) {
                // If a profile with the same values already exists, we don't want to abort the entire
                // transaction. Silently skipping is acceptable.
                insertIgnoreConflicts(profile.copy(name = name))
            }
        }
    }

    private suspend fun findRename(name: String): String? {
        if (get(name) == null) {
            return name
        }
        for (i in 2..100) {
            val newName = "$name ($i)"
            if (get(newName) == null) {
                return newName
            }
        }
        return null
    }
}
