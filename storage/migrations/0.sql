CREATE TABLE paired_device (
    name TEXT NOT NULL,
    mac_address TEXT NOT NULL UNIQUE,
    model TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s'))
);

CREATE INDEX paired_device_created_at ON paired_device (created_at ASC);
