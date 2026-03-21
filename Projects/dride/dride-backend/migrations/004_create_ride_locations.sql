CREATE TABLE ride_locations (
    id          BIGSERIAL PRIMARY KEY,
    ride_id     UUID NOT NULL REFERENCES rides(id),
    user_id     UUID NOT NULL REFERENCES users(id),
    lat         DECIMAL(10,7) NOT NULL,
    lng         DECIMAL(10,7) NOT NULL,
    heading     DECIMAL(5,1),
    speed_kmh   DECIMAL(5,1),
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_ride_locations_ride ON ride_locations(ride_id, recorded_at DESC);
CREATE INDEX idx_ride_locations_geo ON ride_locations USING gist (
    ll_to_earth(lat::float8, lng::float8)
);
