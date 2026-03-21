CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "cube";
CREATE EXTENSION IF NOT EXISTS "earthdistance";

CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phone           VARCHAR(20) UNIQUE NOT NULL,
    name            VARCHAR(100) NOT NULL DEFAULT '',
    email           VARCHAR(255) UNIQUE,
    avatar_url      TEXT,
    role            VARCHAR(10) NOT NULL DEFAULT 'passenger',
    is_driver_active BOOLEAN NOT NULL DEFAULT false,
    driver_vehicle  JSONB,
    wallet_pubkey   VARCHAR(44) UNIQUE NOT NULL,
    rating_avg      DOUBLE PRECISION NOT NULL DEFAULT 5.00,
    rating_count    INTEGER NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_users_driver_active ON users(is_driver_active) WHERE is_driver_active = true;
