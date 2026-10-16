package com.oppzippy.openscq30.features.soundcoredevice.service

/** The number of quick preset slots that get a button in the notification. */
const val QUICK_PRESET_NOTIFICATION_SLOT_COUNT = 2

data class QuickPresetNotificationSlot(val slotIndex: Int, val name: String)

/**
 * Maps the stored quick preset slots to the buttons shown in the notification. Empty slots are skipped, but the
 * remaining slots keep their original index, since that index is what the notification's broadcast uses to look the
 * preset back up in [slotNames].
 */
fun quickPresetNotificationSlots(slotNames: List<String?>): List<QuickPresetNotificationSlot> = slotNames
    .take(QUICK_PRESET_NOTIFICATION_SLOT_COUNT)
    .mapIndexedNotNull { index, name -> name?.let { QuickPresetNotificationSlot(index, it) } }
