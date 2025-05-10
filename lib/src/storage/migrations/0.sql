CREATE TABLE paired_device (
    mac_address TEXT NOT NULL UNIQUE,
    model TEXT NOT NULL,
    is_demo INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s'))
) STRICT;
CREATE INDEX idx_paired_device_created_at ON paired_device (model);

CREATE TABLE quick_preset (
    device_model TEXT NOT NULL,
    name TEXT NOT NULL,
    fields TEXT NOT NULL CHECK(json_valid(fields)),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s'))
) STRICT;
CREATE UNIQUE INDEX idx_quick_preset_device_model_name ON quick_preset (device_model, name);

CREATE TABLE equalizer_profile (
    device_model TEXT NOT NULL,
    name TEXT NOT NULL,
    volume_adjustments TEXT NOT NULL CHECK(json_valid(volume_adjustments)),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s'))
) STRICT;
CREATE UNIQUE INDEX idx_equalizer_profile_name ON equalizer_profile (device_model, name);
CREATE UNIQUE INDEX idx_equalizer_profile_volume_adjustments ON equalizer_profile (device_model, volume_adjustments);
