package com.oppzippy.openscq30.features.statusnotification.storage

import androidx.room.Entity

@Entity(primaryKeys = ["deviceModel", "slotIndex"])
data class QuickPresetSlot(val deviceModel: String, val slotIndex: Int, val name: String)
