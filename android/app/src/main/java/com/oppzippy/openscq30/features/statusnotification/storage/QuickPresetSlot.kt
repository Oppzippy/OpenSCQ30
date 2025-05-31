package com.oppzippy.openscq30.features.statusnotification.storage

import androidx.room.Dao
import androidx.room.Entity
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import com.oppzippy.openscq30.room.AppDatabase
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map
import javax.inject.Singleton

@Entity(primaryKeys = ["deviceModel", "slotIndex"])
data class QuickPresetSlot(val deviceModel: String, val slotIndex: Int, val name: String)

@Dao
abstract class QuickPresetSlotDao {
    @Query("SELECT * FROM QuickPresetSlot WHERE deviceModel = :deviceModel")
    abstract fun all(deviceModel: String): Flow<List<QuickPresetSlot>>

    fun allNames(deviceModel: String): Flow<List<String?>> = all(deviceModel).map { slots ->
        val max = slots.maxBy { it.slotIndex }.slotIndex
        val slotsByIndex = slots.associateBy { it.slotIndex }
        (0..max).map { slotsByIndex[it]?.name }
    }

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    abstract suspend fun upsert(quickPresetSlot: QuickPresetSlot)

    @Query("DELETE FROM QuickPresetSlot WHERE deviceModel = :deviceModel AND slotIndex = :index")
    abstract suspend fun delete(deviceModel: String, index: Int)
}

@Module
@InstallIn(SingletonComponent::class)
class QuickPresetSlotDaoModule {
    @Provides
    @Singleton
    fun provideQuickPresetSlotDao(database: AppDatabase): QuickPresetSlotDao = database.quickPresetSlotDao()
}
