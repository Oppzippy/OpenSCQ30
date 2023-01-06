package com.oppzippy.openscq30

import com.oppzippy.openscq30.lib.SoundcoreDeviceRegistry

private var _soundcoreDeviceRegistry: SoundcoreDeviceRegistry? = null
val soundcoreDeviceRegistry: SoundcoreDeviceRegistry
    get(): SoundcoreDeviceRegistry {
        if (_soundcoreDeviceRegistry == null) {
            _soundcoreDeviceRegistry = SoundcoreDeviceRegistry()
        }
        return _soundcoreDeviceRegistry as SoundcoreDeviceRegistry
    }
