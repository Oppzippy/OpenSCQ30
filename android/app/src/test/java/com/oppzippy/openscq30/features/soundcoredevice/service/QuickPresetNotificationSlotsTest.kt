package com.oppzippy.openscq30.features.soundcoredevice.service

import org.junit.Assert.assertEquals
import org.junit.Test

class QuickPresetNotificationSlotsTest {
    @Test
    fun firstSlotOnlyKeepsIndexZero() {
        assertEquals(
            listOf(QuickPresetNotificationSlot(0, "preset")),
            quickPresetNotificationSlots(listOf("preset", null)),
        )
    }

    @Test
    fun secondSlotOnlyKeepsIndexOne() {
        assertEquals(
            listOf(QuickPresetNotificationSlot(1, "preset")),
            quickPresetNotificationSlots(listOf(null, "preset")),
        )
    }

    @Test
    fun listTruncatedAfterLastStoredSlotKeepsIndexZero() {
        // allNames() only returns entries up to the highest stored slot, so [preset, null] arrives as [preset]
        assertEquals(
            listOf(QuickPresetNotificationSlot(0, "preset")),
            quickPresetNotificationSlots(listOf("preset")),
        )
    }

    @Test
    fun bothSlotsFilledKeepTheirIndices() {
        assertEquals(
            listOf(QuickPresetNotificationSlot(0, "first"), QuickPresetNotificationSlot(1, "second")),
            quickPresetNotificationSlots(listOf("first", "second")),
        )
    }

    @Test
    fun bothSlotsEmptyProduceNoButtons() {
        assertEquals(emptyList<QuickPresetNotificationSlot>(), quickPresetNotificationSlots(listOf(null, null)))
        assertEquals(emptyList<QuickPresetNotificationSlot>(), quickPresetNotificationSlots(emptyList()))
    }

    @Test
    fun slotsBeyondTheLimitAreIgnored() {
        assertEquals(
            listOf(QuickPresetNotificationSlot(1, "second")),
            quickPresetNotificationSlots(listOf(null, "second", "third")),
        )
    }

    @Test
    fun slotIndexMatchesThePositionInTheOriginalList() {
        val slotNames = listOf(null, "preset")
        val slot = quickPresetNotificationSlots(slotNames).single()
        assertEquals(slot.name, slotNames[slot.slotIndex])
    }

    @Test
    fun retainExistingKeepsExistingPresetsAtTheirIndices() {
        assertEquals(
            listOf("first", "second"),
            retainExistingQuickPresets(listOf("first", "second"), setOf("first", "second", "other")),
        )
    }

    @Test
    fun retainExistingBlanksDeletedPresetsWithoutShiftingOtherSlots() {
        assertEquals(
            listOf<String?>(null, "second"),
            retainExistingQuickPresets(listOf("first", "second"), setOf("second")),
        )
        assertEquals(
            listOf("first", null),
            retainExistingQuickPresets(listOf("first", "second"), setOf("first")),
        )
    }

    @Test
    fun retainExistingKeepsEmptySlotsEmpty() {
        assertEquals(
            listOf<String?>(null, "second"),
            retainExistingQuickPresets(listOf(null, "second"), setOf("second")),
        )
        assertEquals(
            listOf<String?>(null, null),
            retainExistingQuickPresets(listOf("first", "second"), emptySet()),
        )
    }
}
