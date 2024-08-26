package com.oppzippy.openscq30.ui.importexport

import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.os.Build
import android.widget.Toast
import androidx.core.content.getSystemService
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.oppzippy.openscq30.R
import com.oppzippy.openscq30.features.equalizer.storage.CustomProfileDao
import com.oppzippy.openscq30.ui.importexport.CustomProfile as ImportExportCustomProfile
import dagger.hilt.android.lifecycle.HiltViewModel
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.launch

@HiltViewModel
class ImportExportViewModel @Inject constructor(
    @ApplicationContext val context: Context,
    val customProfileDao: CustomProfileDao,
) : ViewModel() {
    val customProfiles =
        customProfileDao.all().map { profiles -> profiles.map { ImportExportCustomProfile(it) } }
    private val _stateStack = MutableStateFlow<List<ImportExportState>>(emptyList())
    val stateStack = _stateStack.asStateFlow()

    fun pushState(state: ImportExportState, currentIndex: Int): Int {
        // Invalidate future states
        val newStack = _stateStack.value.take(currentIndex + 1).toMutableList()
        newStack.add(state)
        _stateStack.value = newStack
        return newStack.size - 1
    }

    fun setState(state: ImportExportState, currentIndex: Int): Int {
        // Invalidate current and future states
        val newStack = _stateStack.value.take(currentIndex.coerceAtLeast(0)).toMutableList()
        newStack.add(state)
        _stateStack.value = newStack
        return newStack.size - 1
    }

    fun resetState(): Int {
        _stateStack.value = emptyList()
        return -1
    }

    fun copyToClipboard(text: String) {
        val manager = context.getSystemService<ClipboardManager>() ?: return
        val data = ClipData.newPlainText(
            context.getString(R.string.openscq30_exported_profiles),
            text,
        )
        manager.setPrimaryClip(data)
        // Older android versions don't show a notification when copying
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.S_V2) {
            Toast.makeText(context, context.getString(R.string.copied), Toast.LENGTH_SHORT).show()
        }
    }

    fun importCustomProfiles(profiles: List<ImportExportCustomProfile>, overwrite: Boolean) {
        viewModelScope.launch {
            if (overwrite) {
                importAndOverwriteCustomProfiles(profiles)
            } else {
                importAndRenameCustomProfiles(profiles)
            }
        }
    }

    private suspend fun importAndOverwriteCustomProfiles(profiles: List<ImportExportCustomProfile>) {
        customProfileDao.upsertAll(profiles.map { it.toStorageCustomProfile() })
    }

    private suspend fun importAndRenameCustomProfiles(profiles: List<ImportExportCustomProfile>) {
        customProfileDao.insertAndRename(profiles.map { it.toStorageCustomProfile() })
    }
}
