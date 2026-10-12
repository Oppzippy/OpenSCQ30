-- Settings that a device accepts but cannot report back or does not persist itself,
-- so openscq30 remembers the last value and reapplies it on connect.
CREATE TABLE device_model_setting (
    device_model TEXT NOT NULL,
    setting_id TEXT NOT NULL,
    value TEXT NOT NULL CHECK(json_valid(value)),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s'))
) STRICT;
CREATE UNIQUE INDEX idx_device_model_setting_device_model_setting_id ON device_model_setting (device_model, setting_id);
