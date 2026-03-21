CREATE TABLE ratings (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ride_id     UUID NOT NULL REFERENCES rides(id),
    rater_id    UUID NOT NULL REFERENCES users(id),
    rated_id    UUID NOT NULL REFERENCES users(id),
    score       SMALLINT NOT NULL CHECK (score BETWEEN 1 AND 5),
    comment     TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (ride_id, rater_id)
);

CREATE INDEX idx_ratings_rated ON ratings(rated_id, created_at DESC);
