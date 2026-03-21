use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::auth::middleware::AuthUser;
use crate::error::AppError;
use crate::models::user::{self, UpdateUserRequest, UserResponse};
use crate::AppState;

pub async fn get_me(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    let db_user = user::find_by_id(&state.pool, auth.claims.sub)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;
    Ok(Json(UserResponse::from(db_user)))
}

pub async fn update_me(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    // Validate role if provided
    if let Some(ref role) = body.role {
        if role != "passenger" && role != "driver" {
            return Err(AppError::Validation("Role must be 'passenger' or 'driver'".into()));
        }
    }

    let updated = user::update(&state.pool, auth.claims.sub, &body).await?;
    Ok(Json(UserResponse::from(updated)))
}

pub async fn delete_me(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<StatusCode, AppError> {
    user::soft_delete(&state.pool, auth.claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}
