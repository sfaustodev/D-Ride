use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::auth::jwt::create_token;
use crate::auth::otp::{generate_otp, send_otp_sms};
use crate::error::AppError;
use crate::models::user::{self, UserResponse};
use crate::models::wallet;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct OtpRequestBody {
    pub phone: String,
}

#[derive(Debug, Serialize)]
pub struct OtpRequestResponse {
    pub message: String,
    pub expires_in: u64,
}

pub async fn request_otp(
    State(state): State<AppState>,
    Json(body): Json<OtpRequestBody>,
) -> Result<Json<OtpRequestResponse>, AppError> {
    if body.phone.is_empty() || !body.phone.starts_with('+') {
        return Err(AppError::Validation("Invalid phone number".into()));
    }

    let code = generate_otp();

    // Store OTP in Redis with expiry
    let mut redis_conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis error: {e}")))?;

    let redis_key = format!("otp:{}", body.phone);
    redis::AsyncCommands::set_ex::<_, _, ()>(&mut redis_conn, &redis_key, &code, state.config.otp_expiry_seconds)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis error: {e}")))?;

    send_otp_sms(
        &body.phone,
        &code,
        &state.config.twilio_account_sid,
        &state.config.twilio_auth_token,
        &state.config.twilio_from_number,
    )
    .await
    .map_err(|e| AppError::Internal(e))?;

    tracing::debug!("OTP for {} stored in Redis", body.phone);

    Ok(Json(OtpRequestResponse {
        message: "OTP sent".into(),
        expires_in: state.config.otp_expiry_seconds,
    }))
}

#[derive(Debug, Deserialize)]
pub struct OtpVerifyBody {
    pub phone: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct OtpVerifyResponse {
    pub token: String,
    pub user: UserResponse,
    pub is_new_user: bool,
}

pub async fn verify_otp(
    State(state): State<AppState>,
    Json(body): Json<OtpVerifyBody>,
) -> Result<Json<OtpVerifyResponse>, AppError> {
    // Retrieve OTP from Redis
    let mut redis_conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis error: {e}")))?;

    let redis_key = format!("otp:{}", body.phone);
    let stored_code: Option<String> = redis::AsyncCommands::get(&mut redis_conn, &redis_key)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis error: {e}")))?;

    let stored_code = stored_code.ok_or(AppError::Validation("OTP expired or not found".into()))?;

    if stored_code != body.code {
        return Err(AppError::Validation("Invalid OTP code".into()));
    }

    // Delete used OTP
    let _: () = redis::AsyncCommands::del(&mut redis_conn, &redis_key)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Redis error: {e}")))?;

    // Find or create user
    let existing = user::find_by_phone(&state.pool, &body.phone).await?;
    let (db_user, is_new) = match existing {
        Some(u) => (u, false),
        None => {
            // Generate a placeholder wallet pubkey for new user
            // In production, the iOS app sends the pubkey it generated on-device
            let pubkey = format!("wallet_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
            let new_user = user::create(&state.pool, &body.phone, &pubkey).await?;
            // Create wallet record
            wallet::create(&state.pool, new_user.id, &pubkey, "placeholder_encrypted_sk").await?;
            (new_user, true)
        }
    };

    let token = create_token(db_user.id, &db_user.role, &state.config.jwt_secret)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("JWT error: {e}")))?;

    Ok(Json(OtpVerifyResponse {
        token,
        user: UserResponse::from(db_user),
        is_new_user: is_new,
    }))
}
