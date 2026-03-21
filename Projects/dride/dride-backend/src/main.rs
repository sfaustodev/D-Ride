use axum::middleware;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use dride_backend::auth::middleware::auth_middleware;
use dride_backend::config::AppConfig;
use dride_backend::{db, handlers, AppState};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env();
    let pool = db::create_pool(&config).await;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Migrations applied successfully");

    let redis = redis::Client::open(config.redis_url.clone())
        .expect("Failed to create Redis client");

    let state = AppState {
        pool,
        redis,
        config: config.clone(),
    };

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/auth/otp/request", post(handlers::auth::request_otp))
        .route("/auth/otp/verify", post(handlers::auth::verify_otp));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/users/me", get(handlers::users::get_me))
        .route("/users/me", patch(handlers::users::update_me))
        .route("/users/me", delete(handlers::users::delete_me))
        .layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .nest("/v1", public_routes.merge(protected_routes))
        .route("/health", get(|| async { "ok" }))
        .layer(axum::Extension(config.jwt_secret.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(state);

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Starting dRide backend on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server error");
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    tracing::info!("Shutting down gracefully...");
}
