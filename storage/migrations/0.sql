CREATE TABLE paired_device (
    name TEXT NOT NULL,
    mac_address TEXT NOT NULL UNIQUE,
    model TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s'))
);
