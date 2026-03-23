use std::time::Duration;

use crate::AppState;

const RIDE_TIMEOUT_SECONDS: i64 = 300; // 5 minutes
const CHECK_INTERVAL: Duration = Duration::from_secs(30);

/// Periodically expire stale rides that haven't been accepted within 5 minutes
pub async fn run(state: AppState) {
    tracing::info!("expire_rides job started (interval: {:?})", CHECK_INTERVAL);

    loop {
        tokio::time::sleep(CHECK_INTERVAL).await;

        match expire_stale_rides(&state).await {
            Ok(count) => {
                if count > 0 {
                    tracing::info!("Expired {} stale rides", count);
                }
            }
            Err(e) => {
                tracing::error!("expire_rides job error: {e}");
            }
        }
    }
}

async fn expire_stale_rides(state: &AppState) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE rides
        SET status = 'expired', updated_at = now()
        WHERE status = 'requested'
          AND created_at < now() - make_interval(secs => $1)
        "#,
    )
    .bind(RIDE_TIMEOUT_SECONDS as f64)
    .execute(&state.pool)
    .await?;

    Ok(result.rows_affected())
}
