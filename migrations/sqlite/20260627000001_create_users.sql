-- users stores OIDC-authenticated users known to issueflow.
CREATE TABLE IF NOT EXISTS users (
    -- Stable primary key for the local user record.
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    -- OIDC subject identifier used as the external identity key.
    sub        TEXT NOT NULL UNIQUE,
    -- Display name from the OIDC profile.
    name       TEXT NOT NULL DEFAULT '',
    -- Email from the OIDC profile.
    email      TEXT NOT NULL DEFAULT '',
    -- Timestamp when the user record was first created.
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
