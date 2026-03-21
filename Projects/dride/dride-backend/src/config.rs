use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub database_max_connections: u32,
    pub redis_url: String,
    pub jwt_secret: String,
    pub otp_expiry_seconds: u64,
    pub twilio_account_sid: String,
    pub twilio_auth_token: String,
    pub twilio_from_number: String,
    pub solana_rpc_url: String,
    pub escrow_program_id: String,
    pub protocol_wallet: String,
    pub coingecko_api_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("PORT must be a number"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "20".into())
                .parse()
                .expect("DATABASE_MAX_CONNECTIONS must be a number"),
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".into()),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            otp_expiry_seconds: env::var("OTP_EXPIRY_SECONDS")
                .unwrap_or_else(|_| "300".into())
                .parse()
                .expect("OTP_EXPIRY_SECONDS must be a number"),
            twilio_account_sid: env::var("TWILIO_ACCOUNT_SID").unwrap_or_default(),
            twilio_auth_token: env::var("TWILIO_AUTH_TOKEN").unwrap_or_default(),
            twilio_from_number: env::var("TWILIO_FROM_NUMBER").unwrap_or_default(),
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.devnet.solana.com".into()),
            escrow_program_id: env::var("ESCROW_PROGRAM_ID").unwrap_or_default(),
            protocol_wallet: env::var("PROTOCOL_WALLET").unwrap_or_default(),
            coingecko_api_url: env::var("COINGECKO_API_URL")
                .unwrap_or_else(|_| "https://api.coingecko.com/api/v3".into()),
        }
    }
}
