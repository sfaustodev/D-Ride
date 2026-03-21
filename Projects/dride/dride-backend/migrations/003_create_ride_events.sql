CREATE TABLE ride_events (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ride_id     UUID NOT NULL REFERENCES rides(id),
    event_type  VARCHAR(30) NOT NULL,
    actor_id    UUID REFERENCES users(id),
    metadata    JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_ride_events_ride ON ride_events(ride_id, created_at DESC);
