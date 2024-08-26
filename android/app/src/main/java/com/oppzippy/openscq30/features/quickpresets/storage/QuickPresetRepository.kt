package com.oppzippy.openscq30.features.quickpresets.storage

import java.util.UUID
import javax.inject.Inject
import javax.inject.Singleton
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.combine

@Singleton
class QuickPresetRepository @Inject constructor(private val quickPresetDao: QuickPresetDao) {
    suspend fun insert(quickPreset: QuickPreset) {
        quickPresetDao.insert(quickPreset)
    }

    suspend fun insertFallback(fallbackQuickPreset: FallbackQuickPreset) {
        quickPresetDao.insertFallback(fallbackQuickPreset)
    }

    suspend fun delete(deviceBleServiceUuid: UUID, index: Int) {
        quickPresetDao.delete(deviceBleServiceUuid, index)
    }

    suspend fun getForDevice(deviceBleServiceUuid: UUID): List<QuickPreset> {
        val presets = quickPresetDao.getForDevice(deviceBleServiceUuid)
        val fallbacks = quickPresetDao.getFallbacks()
        return (0..1).map { i ->
            presets.firstOrNull { it.index == i } ?: fallbacks.firstOrNull { it.index == i }
                ?.toQuickPreset(deviceBleServiceUuid) ?: QuickPreset(deviceBleServiceUuid, i)
        }
    }

    fun getNamesForDevice(deviceBleServiceUuid: UUID): Flow<List<String?>> {
        val namesFlow = quickPresetDao.allNames(deviceBleServiceUuid)
        val fallbacksFlow = quickPresetDao.allFallbackNames()
        return namesFlow.combine(fallbacksFlow) { names, fallbacks ->
            (0..1).map { i ->
                names.getOrNull(i)?.name ?: fallbacks.getOrNull(i)?.name
            }
        }
    }
}
