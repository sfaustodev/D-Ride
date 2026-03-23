use crate::error::AppError;

const CACHE_KEY: &str = "sol_brl_rate";
const CACHE_TTL: u64 = 60; // seconds

/// Fetch SOL/BRL rate from Redis cache or CoinGecko
pub async fn get_sol_brl_rate(
    redis: &redis::Client,
    coingecko_url: &str,
) -> Result<f64, AppError> {
    // Try cache first
    if let Ok(mut conn) = redis.get_multiplexed_async_connection().await {
        if let Ok(cached) = redis::AsyncCommands::get::<_, Option<String>>(&mut conn, CACHE_KEY).await {
            if let Some(val) = cached {
                if let Ok(rate) = val.parse::<f64>() {
                    return Ok(rate);
                }
            }
        }
    }

    // Fetch from CoinGecko
    let url = format!("{}/simple/price?ids=solana&vs_currencies=brl", coingecko_url);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("CoinGecko request failed: {e}")))?;

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("CoinGecko parse failed: {e}")))?;

    let rate = json["solana"]["brl"]
        .as_f64()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("CoinGecko: missing solana.brl field")))?;

    // Cache in Redis
    if let Ok(mut conn) = redis.get_multiplexed_async_connection().await {
        let _: Result<(), _> =
            redis::AsyncCommands::set_ex(&mut conn, CACHE_KEY, rate.to_string(), CACHE_TTL).await;
    }

    Ok(rate)
}

/// Fallback rate for when CoinGecko is unreachable and cache is empty
pub const FALLBACK_RATE: f64 = 83.0;

/// Get rate with fallback
pub async fn get_sol_brl_rate_safe(redis: &redis::Client, coingecko_url: &str) -> f64 {
    get_sol_brl_rate(redis, coingecko_url)
        .await
        .unwrap_or(FALLBACK_RATE)
}
