CREATE TABLE IF NOT EXISTS users (
    id         BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    sub        TEXT NOT NULL UNIQUE,
    name       TEXT NOT NULL DEFAULT '',
    email      TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE users IS 'OIDC-authenticated users';
COMMENT ON COLUMN users.id IS 'primary key';
COMMENT ON COLUMN users.sub IS 'OIDC subject identifier';
COMMENT ON COLUMN users.name IS 'display name from OIDC profile';
COMMENT ON COLUMN users.email IS 'email from OIDC profile';
COMMENT ON COLUMN users.created_at IS 'first login timestamp';
