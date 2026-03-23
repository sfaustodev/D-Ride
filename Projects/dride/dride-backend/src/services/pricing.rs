/// Porto Seguro MVP fare constants
const BASE_FARE_BRL: f64 = 3.50;
const PER_KM_RATE: f64 = 2.00;
const PER_MIN_RATE: f64 = 0.30;
const MINIMUM_FARE_BRL: f64 = 6.00;
const PROTOCOL_FEE_BPS: i32 = 1000; // 10%
const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

pub struct FareEstimate {
    pub distance_km: f64,
    pub duration_min: i32,
    pub fare_brl: f64,
    pub fare_sol: f64,
    pub fare_lamports: i64,
    pub protocol_fee_bps: i32,
    pub sol_brl_rate: f64,
}

/// Calculate fare from distance/duration and current SOL/BRL rate
pub fn calculate_fare(distance_km: f64, duration_min: i32, sol_brl_rate: f64) -> FareEstimate {
    let raw_fare = BASE_FARE_BRL + (distance_km * PER_KM_RATE) + (duration_min as f64 * PER_MIN_RATE);
    let fare_brl = raw_fare.max(MINIMUM_FARE_BRL);
    let fare_sol = fare_brl / sol_brl_rate;
    let fare_lamports = (fare_sol * LAMPORTS_PER_SOL) as i64;

    FareEstimate {
        distance_km,
        duration_min,
        fare_brl,
        fare_sol,
        fare_lamports,
        protocol_fee_bps: PROTOCOL_FEE_BPS,
        sol_brl_rate,
    }
}

/// Haversine distance between two lat/lng points in km
pub fn haversine_km(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let r = 6371.0; // Earth radius in km
    let dlat = (lat2 - lat1).to_radians();
    let dlng = (lng2 - lng1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlng / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    r * c
}

/// Estimate duration from distance (simple heuristic: 25 km/h average in city)
pub fn estimate_duration_min(distance_km: f64) -> i32 {
    ((distance_km / 25.0) * 60.0).ceil() as i32
}
