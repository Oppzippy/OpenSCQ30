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

    suspend fun delete(model: String, index: Int) {
        quickPresetDao.delete(model, index)
    }

    suspend fun getForDevice(deviceModel: String): List<QuickPreset> {
        val presets = quickPresetDao.getForDevice(deviceModel)
        val fallbacks = quickPresetDao.getFallbacks()
        return (0..1).map { i ->
            presets.firstOrNull { it.index == i } ?: fallbacks.firstOrNull { it.index == i }
                ?.toQuickPreset(deviceModel) ?: QuickPreset(id = null, deviceModel = deviceModel, index = i)
        }
    }

    fun getNamesForDevice(deviceModel: String): Flow<List<String?>> {
        val namesFlow = quickPresetDao.allNames(deviceModel)
        val fallbacksFlow = quickPresetDao.allFallbackNames()
        return namesFlow.combine(fallbacksFlow) { names, fallbacks ->
            (0..1).map { i ->
                names.getOrNull(i)?.name ?: fallbacks.getOrNull(i)?.name
            }
        }
    }

    suspend fun migrateBleServiceUuids(deviceModel: String, deviceBleServiceUuid: UUID) {
        val presets = quickPresetDao.getForDeviceByServiceUuid(deviceBleServiceUuid)
        presets.forEach {
            insert(
                it.copy(
                    deviceModel = deviceModel,
                    deviceBleServiceUuid = null,
                ),
            )
        }
    }
}
