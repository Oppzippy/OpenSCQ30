package com.oppzippy.openscq30.features.whatsnew

import com.oppzippy.openscq30.features.equalizer.storage.LegacyEqualizerProfileDao
import javax.inject.Inject
import kotlinx.coroutines.flow.map

class Version2BreakingChangesMessage @Inject constructor(
    val store: WhatsNewStore,
    legacyEqualizerProfileDao: LegacyEqualizerProfileDao,
) {
    // No point in showing breaking changes if the user didn't make use of the feature that requires manual migration
    val shouldShow =
        store.version2BreakingChangesMessageShown.map { shown -> !shown && legacyEqualizerProfileDao.count() != 0 }

    fun setShown() = store.setVersion2BreakingChangesMessageShown(true)
}
