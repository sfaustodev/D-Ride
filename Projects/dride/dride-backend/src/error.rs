use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Validation: {0}")]
    Validation(String),

    #[error("Ride status invalid: cannot {action} from {current}")]
    InvalidRideTransition { current: String, action: String },

    #[error("Escrow error: {0}")]
    Escrow(String),

    #[error("Database error")]
    Sqlx(#[from] sqlx::Error),

    #[error("Internal error")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::NotFound(m) => (StatusCode::NOT_FOUND, m.clone()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            Self::Validation(m) => (StatusCode::BAD_REQUEST, m.clone()),
            Self::InvalidRideTransition { .. } => (StatusCode::CONFLICT, self.to_string()),
            Self::Escrow(m) => (StatusCode::BAD_GATEWAY, m.clone()),
            Self::Sqlx(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".into(),
            ),
            Self::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error".into(),
            ),
        };
        tracing::error!("AppError: {self}");
        (status, Json(json!({ "error": msg }))).into_response()
    }
}
