CREATE TABLE rides (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    passenger_id      UUID NOT NULL REFERENCES users(id),
    driver_id         UUID REFERENCES users(id),
    status            VARCHAR(20) NOT NULL DEFAULT 'requested',
    pickup_lat        DECIMAL(10,7) NOT NULL,
    pickup_lng        DECIMAL(10,7) NOT NULL,
    pickup_address    TEXT NOT NULL,
    dropoff_lat       DECIMAL(10,7) NOT NULL,
    dropoff_lng       DECIMAL(10,7) NOT NULL,
    dropoff_address   TEXT NOT NULL,
    distance_km       DECIMAL(6,2) NOT NULL,
    duration_min      INTEGER NOT NULL,
    fare_lamports     BIGINT NOT NULL,
    fare_brl          DECIMAL(10,2) NOT NULL,
    protocol_fee_bps  INTEGER NOT NULL DEFAULT 1000,
    escrow_pubkey     VARCHAR(44),
    escrow_tx_sig     VARCHAR(88),
    release_tx_sig    VARCHAR(88),
    started_at        TIMESTAMPTZ,
    completed_at      TIMESTAMPTZ,
    cancelled_at      TIMESTAMPTZ,
    cancelled_by      VARCHAR(10),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_rides_status ON rides(status) WHERE status IN ('requested', 'accepted', 'active');
CREATE INDEX idx_rides_passenger ON rides(passenger_id, created_at DESC);
CREATE INDEX idx_rides_driver ON rides(driver_id, created_at DESC);
