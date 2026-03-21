CREATE TABLE wallets (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL UNIQUE REFERENCES users(id),
    pubkey          VARCHAR(44) UNIQUE NOT NULL,
    encrypted_sk    TEXT NOT NULL,
    key_version     INTEGER NOT NULL DEFAULT 1,
    balance_cached  BIGINT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
