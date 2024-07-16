package com.oppzippy.openscq30.ui.importexport

import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfile as StorageCustomProfile

@Serializable
data class CustomProfile(
    val name: String,
    val volumeAdjustments: List<Double>,
) {
    constructor(dbProfile: StorageCustomProfile) : this(
        dbProfile.name,
        dbProfile.getVolumeAdjustments(),
    )

    fun toStorageCustomProfile(): StorageCustomProfile {
        return StorageCustomProfile(
            name,
            volumeAdjustments[0],
            volumeAdjustments[1],
            volumeAdjustments[2],
            volumeAdjustments[3],
            volumeAdjustments[4],
            volumeAdjustments[5],
            volumeAdjustments[6],
            volumeAdjustments[7],
        )
    }
}

sealed class ImportExportState {
    data class ExportCustomProfiles(val state: ExportCustomProfilesState) : ImportExportState()
    data class ImportCustomProfiles(val state: ImportCustomProfilesState) : ImportExportState()
}

sealed class ExportCustomProfilesState {
    data class ProfileSelection(
        val customProfiles: List<CustomProfile>,
        val selectedProfiles: List<Boolean>,
    ) : ExportCustomProfilesState() {
        fun next(): CopyToClipboard {
            val filteredProfiles =
                customProfiles
                    .filterIndexed { index, _ -> selectedProfiles[index] }
            val json = Json.encodeToString(filteredProfiles)
            return CopyToClipboard(json)
        }
    }

    data class CopyToClipboard(
        val profileString: String,
    ) : ExportCustomProfilesState()
}

sealed class ImportCustomProfilesState {
    data class StringInput(
        val profileString: String = "",
        val exception: Exception? = null,
    ) : ImportCustomProfilesState() {
        fun next(): ImportCustomProfilesState {
            return try {
                val profiles = Json.decodeFromString<List<CustomProfile>>(profileString)
                ImportOptions(
                    profiles,
                    List(profiles.size) { false },
                    List(profiles.size) { null },
                )
            } catch (ex: Exception) {
                copy(exception = ex)
            }
        }
    }

    data class ImportOptions(
        val profiles: List<CustomProfile>,
        val selection: List<Boolean>,
        val rename: List<String?>,
        val overwrite: Boolean = false,
    ) : ImportCustomProfilesState() {
        fun getFilteredProfiles(): List<CustomProfile> {
            return profiles
                .filterIndexed { index, _ -> selection[index] }
                .mapIndexed { index, customProfile ->
                    rename[index]?.let {
                        customProfile.copy(name = it)
                    } ?: customProfile
                }
        }
    }
}
