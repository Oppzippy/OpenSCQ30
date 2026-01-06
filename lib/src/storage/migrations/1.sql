ALTER TABLE paired_device ADD COLUMN last_connected_at INTEGER;
CREATE INDEX idx_paired_device_last_connected_at ON paired_device (last_connected_at);
