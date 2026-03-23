pub mod expire_rides;

use crate::AppState;

/// Spawn all background jobs
pub fn spawn_jobs(state: AppState) {
    tokio::spawn(expire_rides::run(state));
}
