use axum::middleware;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use dride_backend::auth::jwt::create_token;
use dride_backend::auth::middleware::auth_middleware;
use dride_backend::config::AppConfig;
use dride_backend::handlers;
use dride_backend::AppState;

pub async fn create_test_app() -> Router {
    let (app, _) = create_test_app_with_state().await;
    app
}

pub async fn create_test_app_with_state() -> (Router, AppState) {
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env();
    let pool = dride_backend::db::create_pool(&config).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let redis =
        redis::Client::open(config.redis_url.clone()).expect("Failed to create Redis client");

    let state = AppState {
        pool,
        redis,
        config: config.clone(),
    };

    let public_routes = Router::new()
        .route("/auth/otp/request", post(handlers::auth::request_otp))
        .route("/auth/otp/verify", post(handlers::auth::verify_otp));

    let protected_routes = Router::new()
        .route("/users/me", get(handlers::users::get_me))
        .route("/users/me", patch(handlers::users::update_me))
        .route("/users/me", delete(handlers::users::delete_me))
        .layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .nest("/v1", public_routes.merge(protected_routes))
        .layer(axum::Extension(config.jwt_secret.clone()))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(state.clone());

    (app, state)
}

pub async fn create_test_user_token(state: &AppState) -> String {
    let phone = format!("+55739990{:05}", rand::random::<u32>() % 100_000);
    let pubkey = format!("test_wallet_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));

    let user = dride_backend::models::user::create(&state.pool, &phone, &pubkey)
        .await
        .expect("Failed to create test user");

    create_token(user.id, &user.role, &state.config.jwt_secret)
        .expect("Failed to create token")
}
