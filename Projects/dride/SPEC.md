# dRide — Decentralized Ride-Sharing Platform

## Complete Technical Specification

**Version:** 1.0.0
**Author:** spec-driven development
**Stack:** iOS (Swift 6 / SwiftUI) + Backend (Rust / Axum) + Smart Contract (Solana / Anchor)

---

## 1. Product Overview

### 1.1 Vision
A peer-to-peer ride-sharing app where payments go through a Solana escrow smart contract instead of a corporation. Drivers keep 90% of the fare. The protocol takes a flat 10% fee — transparent, on-chain, auditable by anyone.

### 1.2 MVP Scope (v0.1)
- Single city (Porto Seguro, BA)
- iOS only (passenger + driver in same app, role toggle)
- Solana Devnet for initial testing, then Mainnet
- Embedded wallet (Keychain-backed) for frictionless onboarding
- Manual fare based on distance (no surge pricing in MVP)
- Portuguese language only

### 1.3 User Roles
- **Passenger**: requests rides, pays via crypto
- **Driver**: accepts rides, receives payment on completion
- Users can switch roles via a toggle in settings

---

## 2. Architecture Overview

```
┌─────────────────────────────────────┐
│           iOS App (Swift)           │
│  SwiftUI + MapKit + WalletManager  │
└──────────────┬──────────────────────┘
               │ HTTPS + WSS
┌──────────────▼──────────────────────┐
│        Rust Backend (Axum)          │
│  REST API + WebSocket Hub + Jobs    │
│  PostgreSQL + Redis                 │
└──────────┬─────────┬────────────────┘
           │         │
┌──────────▼───┐ ┌───▼───────────────┐
│  PostgreSQL  │ │  Solana RPC       │
│  (Neon/      │ │  (Helius/         │
│   Supabase)  │ │   QuickNode)      │
└──────────────┘ └───────┬───────────┘
                         │
                ┌────────▼────────────┐
                │  Anchor Program     │
                │  (Escrow + Ride)    │
                └─────────────────────┘
```

---

## 3. Database Schema (PostgreSQL)

### 3.1 Entity Relationship

```
users 1──N rides (as passenger)
users 1──N rides (as driver)
users 1──1 wallets
rides 1──1 escrow_transactions
rides 1──N ride_events
rides 1──N ride_locations
users 1──N ratings
```

### 3.2 Table: `users`

| Column            | Type          | Constraints                    | Notes                         |
|-------------------|---------------|--------------------------------|-------------------------------|
| id                | UUID          | PK, DEFAULT gen_random_uuid()  |                               |
| phone             | VARCHAR(20)   | UNIQUE, NOT NULL               | +55... format                 |
| name              | VARCHAR(100)  | NOT NULL                       |                               |
| email             | VARCHAR(255)  | UNIQUE, nullable               |                               |
| avatar_url        | TEXT          | nullable                       | S3/R2 URL                     |
| role              | VARCHAR(10)   | DEFAULT 'passenger'            | 'passenger' or 'driver'       |
| is_driver_active  | BOOLEAN       | DEFAULT false                  | driver currently accepting?    |
| driver_vehicle    | JSONB         | nullable                       | {make, model, year, plate, color} |
| wallet_pubkey     | VARCHAR(44)   | UNIQUE, NOT NULL               | Solana base58 public key      |
| rating_avg        | DECIMAL(3,2)  | DEFAULT 5.00                   | cached average                |
| rating_count      | INTEGER       | DEFAULT 0                      |                               |
| created_at        | TIMESTAMPTZ   | DEFAULT now()                  |                               |
| updated_at        | TIMESTAMPTZ   | DEFAULT now()                  |                               |

### 3.3 Table: `rides`

| Column              | Type          | Constraints                    | Notes                          |
|---------------------|---------------|--------------------------------|--------------------------------|
| id                  | UUID          | PK, DEFAULT gen_random_uuid()  |                                |
| passenger_id        | UUID          | FK → users.id, NOT NULL        |                                |
| driver_id           | UUID          | FK → users.id, nullable        | null until accepted            |
| status              | VARCHAR(20)   | NOT NULL, DEFAULT 'requested'  | see 3.3.1 status enum          |
| pickup_lat          | DECIMAL(10,7) | NOT NULL                       |                                |
| pickup_lng          | DECIMAL(10,7) | NOT NULL                       |                                |
| pickup_address      | TEXT          | NOT NULL                       | reverse geocoded               |
| dropoff_lat         | DECIMAL(10,7) | NOT NULL                       |                                |
| dropoff_lng         | DECIMAL(10,7) | NOT NULL                       |                                |
| dropoff_address     | TEXT          | NOT NULL                       | reverse geocoded               |
| distance_km         | DECIMAL(6,2)  | NOT NULL                       | MapKit estimate                |
| duration_min        | INTEGER       | NOT NULL                       | MapKit estimate                |
| fare_lamports       | BIGINT        | NOT NULL                       | fare in lamports (1 SOL = 1e9) |
| fare_brl            | DECIMAL(10,2) | NOT NULL                       | display value at request time  |
| protocol_fee_bps    | INTEGER       | DEFAULT 1000                   | 1000 = 10%                     |
| escrow_pubkey       | VARCHAR(44)   | nullable                       | PDA after deposit              |
| escrow_tx_sig       | VARCHAR(88)   | nullable                       | deposit tx signature           |
| release_tx_sig      | VARCHAR(88)   | nullable                       | release/refund tx sig          |
| started_at          | TIMESTAMPTZ   | nullable                       | driver started trip            |
| completed_at        | TIMESTAMPTZ   | nullable                       | trip finished                  |
| cancelled_at        | TIMESTAMPTZ   | nullable                       |                                |
| cancelled_by        | VARCHAR(10)   | nullable                       | 'passenger' or 'driver'        |
| created_at          | TIMESTAMPTZ   | DEFAULT now()                  |                                |
| updated_at          | TIMESTAMPTZ   | DEFAULT now()                  |                                |

#### 3.3.1 Ride Status Enum
```
requested → accepted → deposit_pending → active → completing → completed
                 ↘ cancelled                  ↘ disputed
requested → expired (no driver within 5 min)
```

### 3.4 Table: `ride_events`

| Column     | Type        | Constraints                   | Notes                        |
|------------|-------------|-------------------------------|------------------------------|
| id         | UUID        | PK                            |                              |
| ride_id    | UUID        | FK → rides.id, NOT NULL       | indexed                      |
| event_type | VARCHAR(30) | NOT NULL                      | 'requested','accepted', etc  |
| actor_id   | UUID        | FK → users.id                 | who triggered                |
| metadata   | JSONB       | nullable                      | extra context                |
| created_at | TIMESTAMPTZ | DEFAULT now()                 |                              |

### 3.5 Table: `ride_locations`

| Column     | Type          | Constraints             | Notes                    |
|------------|---------------|-------------------------|--------------------------|
| id         | BIGSERIAL     | PK                      | high volume table        |
| ride_id    | UUID          | FK → rides.id, NOT NULL | indexed                  |
| user_id    | UUID          | FK → users.id, NOT NULL |                          |
| lat        | DECIMAL(10,7) | NOT NULL                |                          |
| lng        | DECIMAL(10,7) | NOT NULL                |                          |
| heading    | DECIMAL(5,1)  | nullable                | compass degrees          |
| speed_kmh  | DECIMAL(5,1)  | nullable                |                          |
| recorded_at| TIMESTAMPTZ   | DEFAULT now()           | indexed for range queries|

### 3.6 Table: `ratings`

| Column     | Type        | Constraints                   |
|------------|-------------|-------------------------------|
| id         | UUID        | PK                            |
| ride_id    | UUID        | FK → rides.id, UNIQUE pair    |
| rater_id   | UUID        | FK → users.id                 |
| rated_id   | UUID        | FK → users.id                 |
| score      | SMALLINT    | CHECK (score BETWEEN 1 AND 5) |
| comment    | TEXT        | nullable                      |
| created_at | TIMESTAMPTZ | DEFAULT now()                 |

### 3.7 Table: `wallets`

| Column          | Type        | Constraints                  | Notes                      |
|-----------------|-------------|------------------------------|----------------------------|
| id              | UUID        | PK                           |                            |
| user_id         | UUID        | FK → users.id, UNIQUE        |                            |
| pubkey          | VARCHAR(44) | UNIQUE, NOT NULL             | Solana public key          |
| encrypted_sk    | TEXT        | NOT NULL                     | AES-256-GCM encrypted     |
| key_version     | INTEGER     | DEFAULT 1                    | for key rotation           |
| balance_cached  | BIGINT      | DEFAULT 0                    | lamports, updated by cron  |
| created_at      | TIMESTAMPTZ | DEFAULT now()                |                            |

### 3.8 Indexes

```sql
CREATE INDEX idx_rides_status ON rides(status) WHERE status IN ('requested', 'accepted', 'active');
CREATE INDEX idx_rides_passenger ON rides(passenger_id, created_at DESC);
CREATE INDEX idx_rides_driver ON rides(driver_id, created_at DESC);
CREATE INDEX idx_ride_locations_ride ON ride_locations(ride_id, recorded_at DESC);
CREATE INDEX idx_ride_locations_geo ON ride_locations USING gist (
  ll_to_earth(lat::float8, lng::float8)
);
CREATE INDEX idx_users_driver_active ON users(is_driver_active) WHERE is_driver_active = true;
CREATE INDEX idx_ratings_rated ON ratings(rated_id, created_at DESC);
```

---

## 4. REST API Specification (Rust / Axum)

**Base URL:** `https://api.dride.app/v1`
**Auth:** Bearer JWT (issued on phone OTP verification)
**Content-Type:** `application/json`

### 4.1 Auth

#### `POST /auth/otp/request`
Send OTP to phone number.
```json
// Request
{ "phone": "+5573999001234" }
// Response 200
{ "message": "OTP sent", "expires_in": 300 }
```

#### `POST /auth/otp/verify`
Verify OTP and return JWT. Creates user + wallet on first login.
```json
// Request
{ "phone": "+5573999001234", "code": "123456" }
// Response 200
{
  "token": "eyJhbG...",
  "user": { "id": "uuid", "name": "", "phone": "+55...", "wallet_pubkey": "ABC..." },
  "is_new_user": true
}
```

### 4.2 Users (CRUD)

#### `GET /users/me`
Returns authenticated user profile.

#### `PATCH /users/me`
Update profile fields.
```json
// Request (all fields optional)
{
  "name": "João Silva",
  "email": "joao@email.com",
  "role": "driver",
  "driver_vehicle": {
    "make": "Honda", "model": "Civic", "year": 2020,
    "plate": "ABC1D23", "color": "Prata"
  }
}
```

#### `DELETE /users/me`
Soft delete account (anonymize PII, keep ride history for disputes).

### 4.3 Rides (CRUD)

#### `POST /rides/estimate`
Get fare estimate before requesting.
```json
// Request
{
  "pickup_lat": -16.4489, "pickup_lng": -39.0648,
  "dropoff_lat": -16.4370, "dropoff_lng": -39.0580
}
// Response 200
{
  "distance_km": 2.3,
  "duration_min": 8,
  "fare_lamports": 150000000,
  "fare_brl": 12.50,
  "fare_sol": 0.15,
  "protocol_fee_bps": 1000,
  "sol_brl_rate": 83.33
}
```

#### `POST /rides`
Request a ride (passenger). Backend creates escrow PDA and returns deposit instruction.
```json
// Request
{
  "pickup_lat": -16.4489, "pickup_lng": -39.0648,
  "pickup_address": "Av. do Descobrimento, 123",
  "dropoff_lat": -16.4370, "dropoff_lng": -39.0580,
  "dropoff_address": "Praia de Taperapuã"
}
// Response 201
{
  "ride": { "id": "uuid", "status": "requested", "fare_lamports": 150000000, ... },
  "escrow": {
    "pubkey": "EscrowPDA...",
    "deposit_tx": "base64_serialized_tx_to_sign"
  }
}
```

#### `POST /rides/:id/deposit-confirm`
Passenger confirms deposit transaction was sent.
```json
// Request
{ "tx_signature": "5UKg3..." }
// Response 200
{ "ride": { "id": "uuid", "status": "deposit_pending" } }
```
Backend verifies tx on-chain, then sets status to `requested` (visible to drivers).

#### `GET /rides/available`
List nearby available rides (driver only). Returns rides within 5km radius.
```json
// Response 200
{
  "rides": [
    {
      "id": "uuid", "pickup_address": "...", "dropoff_address": "...",
      "distance_km": 2.3, "fare_brl": 12.50, "fare_sol": 0.15,
      "pickup_distance_km": 0.8
    }
  ]
}
```

#### `POST /rides/:id/accept`
Driver accepts a ride.
```json
// Response 200
{
  "ride": { "id": "uuid", "status": "accepted", "driver_id": "uuid" },
  "passenger": { "name": "João", "phone": "+55...", "rating_avg": 4.8 }
}
```

#### `POST /rides/:id/start`
Driver confirms pickup, trip starts.
```json
// Response 200
{ "ride": { "id": "uuid", "status": "active", "started_at": "2026-03-..." } }
```

#### `POST /rides/:id/complete`
Driver marks ride as completed. Backend triggers escrow release on-chain.
```json
// Response 200
{
  "ride": { "id": "uuid", "status": "completing" },
  "release_tx": "base64_tx..."
}
```
Backend confirms release on-chain → status becomes `completed`.

#### `POST /rides/:id/cancel`
Either party cancels. Backend triggers escrow refund if deposit exists.
```json
// Request
{ "reason": "changed_plans" }
// Response 200
{ "ride": { "id": "uuid", "status": "cancelled" } }
```
Refund rules:
- Before driver accepts → full refund to passenger
- After accept, before start → full refund (grace period)
- After start → partial refund based on distance traveled (future v0.2)

#### `GET /rides/:id`
Get full ride details.

#### `GET /rides/history`
Paginated ride history for current user.
```
GET /rides/history?page=1&per_page=20&role=passenger
```

### 4.4 Ratings

#### `POST /rides/:id/rate`
```json
// Request
{ "score": 5, "comment": "Ótimo motorista!" }
// Response 201
{ "rating": { "id": "uuid", "score": 5 } }
```

### 4.5 Wallet

#### `GET /wallet/balance`
```json
// Response 200
{ "balance_lamports": 500000000, "balance_sol": 0.5, "balance_brl": 41.50, "sol_brl_rate": 83.00 }
```

#### `POST /wallet/fund`
Generate Solana deposit address + PIX QR (via on-ramp partner, future v0.2).

#### `GET /wallet/transactions`
On-chain transaction history for this wallet.

---

## 5. WebSocket Specification

**Endpoint:** `wss://api.dride.app/v1/ws?token=JWT`

All messages are JSON with `{ "type": "...", "payload": {...} }` format.

### 5.1 Client → Server Events

| Type                | Payload                                    | Description                |
|---------------------|--------------------------------------------|----------------------------|
| `location_update`   | `{ lat, lng, heading, speed_kmh }`         | Driver sends every 3s      |
| `ping`              | `{}`                                       | Keepalive every 30s        |

### 5.2 Server → Client Events

| Type                  | Payload                                    | Who receives   |
|-----------------------|--------------------------------------------|----------------|
| `ride_requested`      | `{ ride summary }`                         | Nearby drivers |
| `ride_accepted`       | `{ ride, driver info }`                    | Passenger      |
| `ride_cancelled`      | `{ ride_id, cancelled_by, reason }`        | Both parties   |
| `ride_started`        | `{ ride_id, started_at }`                  | Passenger      |
| `ride_completed`      | `{ ride_id, release_tx }`                  | Both parties   |
| `driver_location`     | `{ ride_id, lat, lng, heading, eta_min }`  | Passenger      |
| `deposit_confirmed`   | `{ ride_id, escrow_pubkey }`               | Both parties   |
| `escrow_released`     | `{ ride_id, tx_sig, driver_amount }`       | Both parties   |

---

## 6. Rust Backend Structure

### 6.1 Project Layout

```
dride-backend/
├── Cargo.toml
├── .env.example
├── migrations/           # sqlx migrations
│   ├── 001_create_users.sql
│   ├── 002_create_rides.sql
│   ├── 003_create_ride_events.sql
│   ├── 004_create_ride_locations.sql
│   ├── 005_create_ratings.sql
│   └── 006_create_wallets.sql
├── src/
│   ├── main.rs           # Axum server setup, router, graceful shutdown
│   ├── config.rs         # env vars, app config struct
│   ├── error.rs          # AppError enum, IntoResponse impl
│   ├── auth/
│   │   ├── mod.rs
│   │   ├── jwt.rs        # JWT encode/decode, middleware extractor
│   │   ├── otp.rs        # OTP generation, Twilio/SMS integration
│   │   └── middleware.rs  # auth guard layer
│   ├── db/
│   │   ├── mod.rs
│   │   └── pool.rs       # PgPool setup with sqlx
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs       # User struct, FromRow, CRUD queries
│   │   ├── ride.rs       # Ride struct, status transitions, queries
│   │   ├── ride_event.rs
│   │   ├── ride_location.rs
│   │   ├── rating.rs
│   │   └── wallet.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── auth.rs       # POST /auth/otp/*
│   │   ├── users.rs      # GET/PATCH/DELETE /users/me
│   │   ├── rides.rs      # all /rides/* endpoints
│   │   ├── ratings.rs    # POST /rides/:id/rate
│   │   └── wallet.rs     # GET /wallet/*
│   ├── ws/
│   │   ├── mod.rs
│   │   ├── hub.rs        # connection registry, broadcast by geo
│   │   ├── handler.rs    # upgrade, message dispatch
│   │   └── messages.rs   # typed WS message enums
│   ├── solana/
│   │   ├── mod.rs
│   │   ├── client.rs     # RPC client wrapper (solana-client)
│   │   ├── escrow.rs     # build deposit/release/refund instructions
│   │   ├── tx_builder.rs # transaction construction + signing
│   │   └── listener.rs   # on-chain event polling / webhooks
│   ├── services/
│   │   ├── mod.rs
│   │   ├── ride_service.rs    # ride lifecycle orchestration
│   │   ├── matching.rs        # find nearby drivers, notify
│   │   ├── pricing.rs         # fare calculation
│   │   └── location.rs        # geo utilities, distance calc
│   └── jobs/
│       ├── mod.rs
│       ├── expire_rides.rs    # cancel stale requested rides
│       ├── sync_balances.rs   # update cached wallet balances
│       └── confirm_escrow.rs  # poll escrow deposit confirmations
└── tests/
    ├── api/               # integration tests per endpoint
    └── solana/            # escrow program integration tests
```

### 6.2 Key Dependencies (Cargo.toml)

```toml
[dependencies]
axum = { version = "0.8", features = ["ws", "macros"] }
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono", "json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
jsonwebtoken = "9"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

```toml
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
dotenvy = "0.15"
thiserror = "2"
validator = { version = "0.18", features = ["derive"] }
solana-sdk = "2.1"
solana-client = "2.1"
anchor-client = "0.30"
reqwest = { version = "0.12", features = ["json"] }
redis = { version = "0.27", features = ["tokio-comp"] }
rand = "0.8"
argon2 = "0.5"
```

### 6.3 Error Handling Pattern

```rust
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
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".into()),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}
```

---

## 7. Solana Smart Contract (Anchor)

### 7.1 Program: `dride_escrow`

```
programs/dride-escrow/
├── Cargo.toml
├── Xargo.toml
└── src/
    ├── lib.rs            # program entrypoint, declare_id!
    ├── instructions/
    │   ├── mod.rs
    │   ├── create_ride.rs    # passenger deposits SOL into escrow PDA
    │   ├── accept_ride.rs    # driver claims acceptance (optional on-chain)
    │   ├── complete_ride.rs  # release escrow: 90% driver, 10% protocol
    │   ├── cancel_ride.rs    # refund 100% to passenger
    │   └── dispute.rs        # freeze funds, future arbitration (v0.2)
    ├── state/
    │   ├── mod.rs
    │   └── ride_escrow.rs    # account struct
    └── errors.rs             # custom error codes
```

### 7.2 Account: `RideEscrow` (PDA)

Seeds: `["ride", ride_uuid_bytes]`

```rust
#[account]
pub struct RideEscrow {
    pub ride_id: [u8; 16],         // UUID bytes
    pub passenger: Pubkey,          // who deposited
    pub driver: Pubkey,             // assigned driver (Pubkey::default() if none)
    pub amount: u64,                // total fare in lamports
    pub protocol_fee_bps: u16,     // 1000 = 10%
    pub protocol_wallet: Pubkey,   // treasury that receives fee
    pub status: EscrowStatus,       // Created, Active, Completed, Cancelled, Disputed
    pub created_at: i64,            // unix timestamp
    pub bump: u8,                   // PDA bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum EscrowStatus {
    Created,     // SOL deposited, waiting for driver
    Active,      // driver accepted, ride in progress
    Completed,   // funds released to driver + protocol
    Cancelled,   // funds returned to passenger
    Disputed,    // frozen, pending resolution
}
```

### 7.3 Instructions

#### `create_ride`
Passenger deposits SOL into PDA escrow.
```rust
pub fn create_ride(
    ctx: Context<CreateRide>,
    ride_id: [u8; 16],
    amount: u64,
    protocol_fee_bps: u16,
) -> Result<()> {
    require!(amount > 0, DRideError::InvalidAmount);
    require!(protocol_fee_bps <= 5000, DRideError::FeeTooHigh);

    // Transfer SOL from passenger to escrow PDA
    let ix = system_instruction::transfer(
        &ctx.accounts.passenger.key(),
        &ctx.accounts.escrow.key(),
        amount,
    );
    invoke(&ix, &[ctx.accounts.passenger.to_account_info(), ...])?;

    let escrow = &mut ctx.accounts.escrow;
    escrow.ride_id = ride_id;
    escrow.passenger = ctx.accounts.passenger.key();
    escrow.driver = Pubkey::default();
    escrow.amount = amount;
    escrow.protocol_fee_bps = protocol_fee_bps;
    escrow.protocol_wallet = ctx.accounts.protocol_wallet.key();
    escrow.status = EscrowStatus::Created;
    escrow.created_at = Clock::get()?.unix_timestamp;
    escrow.bump = ctx.bumps.escrow;
    Ok(())
}
```

#### `complete_ride`
Backend (authority) releases funds: 90% to driver, 10% to protocol.
```rust
pub fn complete_ride(ctx: Context<CompleteRide>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    require!(escrow.status == EscrowStatus::Active, DRideError::InvalidStatus);

    let fee = escrow.amount * (escrow.protocol_fee_bps as u64) / 10_000;
    let driver_amount = escrow.amount - fee;

    // Transfer driver_amount → driver
    // Transfer fee → protocol_wallet
    // Both via PDA signer seeds

    escrow.status = EscrowStatus::Completed;
    Ok(())
}
```

#### `cancel_ride`
Refund full amount back to passenger.
```rust
pub fn cancel_ride(ctx: Context<CancelRide>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    require!(
        escrow.status == EscrowStatus::Created || escrow.status == EscrowStatus::Active,
        DRideError::InvalidStatus
    );
    // Transfer full escrow.amount → passenger
    escrow.status = EscrowStatus::Cancelled;
    Ok(())
}
```

---

## 8. iOS App Specification (Swift 6 / SwiftUI)

### 8.1 Project Structure

```
dRide/
├── dRide.xcodeproj
├── dRide/
│   ├── App/
│   │   ├── dRideApp.swift            # @main, environment setup
│   │   └── AppState.swift            # @Observable global state
│   ├── Core/
│   │   ├── Network/
│   │   │   ├── APIClient.swift       # generic async request<T>
│   │   │   ├── Endpoints.swift       # all endpoint definitions
│   │   │   ├── AuthInterceptor.swift # JWT injection, refresh
│   │   │   └── WebSocketManager.swift# WSS connection + reconnect
│   │   ├── Wallet/
│   │   │   ├── WalletManager.swift   # Keychain key storage
│   │   │   ├── TransactionBuilder.swift # build + sign Solana txs
│   │   │   └── SolanaRPC.swift       # direct RPC calls
│   │   ├── Location/
│   │   │   ├── LocationService.swift # CLLocationManager wrapper
│   │   │   └── GeocodingService.swift# reverse geocode
│   │   ├── Storage/
│   │   │   └── KeychainHelper.swift  # secure storage wrapper
│   │   └── Extensions/
│   │       ├── CLLocationCoordinate2D+.swift
│   │       └── Date+.swift
│   ├── Features/
│   │   ├── Auth/
│   │   │   ├── Views/
│   │   │   │   ├── PhoneInputView.swift
│   │   │   │   └── OTPVerifyView.swift
│   │   │   └── ViewModels/
│   │   │       └── AuthViewModel.swift
│   │   ├── Passenger/
│   │   │   ├── Views/
│   │   │   │   ├── PassengerHomeView.swift      # map + search destination
│   │   │   │   ├── DestinationSearchView.swift   # autocomplete search
│   │   │   │   ├── RideConfirmView.swift          # confirm fare, deposit
│   │   │   │   ├── WaitingForDriverView.swift     # searching animation
│   │   │   │   ├── RideActiveView.swift           # live map tracking
│   │   │   │   └── RideCompletedView.swift        # rating + receipt
│   │   │   └── ViewModels/
│   │   │       └── PassengerViewModel.swift
│   │   ├── Driver/
│   │   │   ├── Views/
│   │   │   │   ├── DriverHomeView.swift           # toggle online + map
│   │   │   │   ├── RideRequestCard.swift          # incoming ride popup
│   │   │   │   ├── NavigationView.swift           # turn-by-turn
│   │   │   │   └── DriverRideCompleteView.swift   # earning summary
│   │   │   └── ViewModels/
│   │   │       └── DriverViewModel.swift
│   │   ├── Wallet/
│   │   │   ├── Views/
│   │   │   │   ├── WalletView.swift               # balance + history
│   │   │   │   └── FundWalletView.swift           # add funds
│   │   │   └── ViewModels/
│   │   │       └── WalletViewModel.swift
│   │   ├── Profile/
│   │   │   ├── Views/
│   │   │   │   ├── ProfileView.swift
│   │   │   │   └── VehicleFormView.swift
│   │   │   └── ViewModels/
│   │   │       └── ProfileViewModel.swift
│   │   └── RideHistory/
│   │       └── Views/
│   │           └── RideHistoryView.swift
│   ├── Models/
│   │   ├── User.swift
│   │   ├── Ride.swift
│   │   ├── Rating.swift
│   │   └── WalletBalance.swift
│   └── Resources/
│       ├── Assets.xcassets
│       ├── Localizable.xcstrings
│       └── Info.plist
└── dRideTests/
```

### 8.2 Screen Flow

```
Launch → PhoneInputView → OTPVerifyView → PassengerHomeView
                                              │
                              ┌────────────────┼────────────────┐
                              ▼                ▼                ▼
                     Search Destination    Profile/Settings   Toggle to Driver
                              │                                    │
                              ▼                                    ▼
                     RideConfirmView                       DriverHomeView
                    (show fare, sign tx)                  (toggle online)
                              │                                    │
                              ▼                                    ▼
                    WaitingForDriverView               RideRequestCard popup
                              │                                    │
                              ▼                                    ▼
                    RideActiveView ←──── live tracking ────→ NavigationView
                              │                                    │
                              ▼                                    ▼
                    RideCompletedView                  DriverRideCompleteView
                    (rate driver)                      (earnings + rate passenger)
```

### 8.3 Key iOS Models

```swift
struct User: Codable, Identifiable {
    let id: UUID
    var name: String
    let phone: String
    var email: String?
    var avatarUrl: String?
    var role: UserRole
    var isDriverActive: Bool
    var driverVehicle: Vehicle?
    var walletPubkey: String
    var ratingAvg: Double
    var ratingCount: Int
}

enum UserRole: String, Codable { case passenger, driver }

struct Vehicle: Codable {
    var make: String; var model: String; var year: Int
    var plate: String; var color: String
}
```

```swift
struct Ride: Codable, Identifiable {
    let id: UUID
    let passengerId: UUID
    var driverId: UUID?
    var status: RideStatus
    let pickupLat: Double; let pickupLng: Double; let pickupAddress: String
    let dropoffLat: Double; let dropoffLng: Double; let dropoffAddress: String
    let distanceKm: Double; let durationMin: Int
    let fareLamports: Int64; let fareBrl: Double
    let protocolFeeBps: Int
    var escrowPubkey: String?
    var startedAt: Date?; var completedAt: Date?; var cancelledAt: Date?
}

enum RideStatus: String, Codable {
    case requested, accepted, depositPending = "deposit_pending"
    case active, completing, completed, cancelled, expired, disputed
}

struct FareEstimate: Codable {
    let distanceKm: Double; let durationMin: Int
    let fareLamports: Int64; let fareBrl: Double; let fareSol: Double
    let protocolFeeBps: Int; let solBrlRate: Double
}
```

### 8.4 Info.plist Required Keys

```xml
<key>NSLocationWhenInUseUsageDescription</key>
<string>dRide precisa da sua localização para encontrar corridas perto de você.</string>
<key>NSLocationAlwaysAndWhenInUseUsageDescription</key>
<string>dRide usa localização em segundo plano para rastrear corridas ativas.</string>
<key>UIBackgroundModes</key>
<array>
    <string>location</string>
    <string>remote-notification</string>
</array>
```

### 8.5 SPM Dependencies

```swift
// Package.swift / Xcode Add Package
.package(url: "https://github.com/nicklockwood/SwiftFormat", from: "0.54.0"),
.package(url: "https://github.com/nicklockwood/KeychainAccess", from: "4.2.2"), // ??
// Note: use Apple's native Security framework for Keychain in production
// Solana SDK handled via custom thin wrapper to keep app size small
```


---

## 9. Fare Pricing Engine

### 9.1 Formula

```
fare_brl = base_fare + (distance_km × per_km_rate) + (duration_min × per_min_rate)
fare_sol = fare_brl / sol_brl_rate
fare_lamports = fare_sol × 1_000_000_000
```

### 9.2 Rate Table (Porto Seguro MVP)

| Parameter       | Value    | Notes                              |
|-----------------|----------|------------------------------------|
| base_fare       | R$ 3.50  | minimum fare                       |
| per_km_rate     | R$ 2.00  | per kilometer                      |
| per_min_rate    | R$ 0.30  | per estimated minute               |
| minimum_fare    | R$ 6.00  | floor                              |
| protocol_fee    | 10%      | hardcoded in smart contract (1000 bps) |

### 9.3 SOL/BRL Rate
- Fetch from CoinGecko API every 60s, cache in Redis
- Fallback: last known rate if API is down
- Endpoint: `GET https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=brl`
- Rate is locked at ride request time (stored in `rides.fare_lamports`)

---

## 10. Security Specification

### 10.1 Authentication
- Phone OTP via Twilio/SMS (6 digits, 5 min expiry)
- JWT (RS256) with 7-day expiry, refresh on each API call
- JWT payload: `{ sub: user_id, role, iat, exp }`

### 10.2 Wallet Security
- Private keys generated on-device using `SecRandomCopyBytes`
- Encrypted with AES-256-GCM before leaving device
- Encryption key derived from device Secure Enclave (iOS)
- Backend stores ONLY the public key
- Transaction signing happens client-side — backend NEVER sees private keys

### 10.3 Escrow Security
- Smart contract is the sole custodian of funds
- Release requires backend authority signature (multisig in v0.2)
- Cancel can be initiated by passenger OR backend authority
- Funds auto-refund after 24h if ride never starts (on-chain timeout)

### 10.4 API Security
- All endpoints over HTTPS (TLS 1.3)
- Rate limiting: 60 req/min per user, 10 req/min for auth endpoints
- Input validation via `validator` crate on all request bodies
- SQL injection prevention via sqlx parameterized queries
- CORS restricted to app bundle ID in production

---

## 11. Environment Configuration

### 11.1 Backend `.env`

```env
# Server
HOST=0.0.0.0
PORT=8080
RUST_LOG=dride_backend=debug,tower_http=debug

# Database
DATABASE_URL=postgres://dride:password@localhost:5432/dride
DATABASE_MAX_CONNECTIONS=20

# Redis
REDIS_URL=redis://localhost:6379

# Auth
JWT_SECRET=your-rs256-private-key-pem
JWT_PUBLIC_KEY=your-rs256-public-key-pem
OTP_EXPIRY_SECONDS=300
TWILIO_ACCOUNT_SID=ACxxxxxxxxxx
TWILIO_AUTH_TOKEN=xxxxxxxxxx
TWILIO_FROM_NUMBER=+1234567890

# Solana
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_WS_URL=wss://api.devnet.solana.com
ESCROW_PROGRAM_ID=DRide1111111111111111111111111111111111111
PROTOCOL_WALLET=YourProtocolWalletPubkey
BACKEND_AUTHORITY_KEYPAIR=/path/to/authority.json

# External APIs
COINGECKO_API_URL=https://api.coingecko.com/api/v3
```

### 11.2 iOS Configuration

```swift
// Config.swift
enum AppConfig {
    #if DEBUG
    static let apiBaseURL = "http://localhost:8080/v1"
    static let wsBaseURL = "ws://localhost:8080/v1/ws"
    static let solanaCluster: SolanaCluster = .devnet
    #else
    static let apiBaseURL = "https://api.dride.app/v1"
    static let wsBaseURL = "wss://api.dride.app/v1/ws"
    static let solanaCluster: SolanaCluster = .mainnetBeta
    #endif

    static let escrowProgramId = "DRide1111..."
    static let protocolFeeBps: UInt16 = 1000
    static let locationUpdateInterval: TimeInterval = 3.0
    static let rideSearchRadiusKm: Double = 5.0
    static let rideExpirySeconds: Int = 300
}
```

---

## 12. Deployment

### 12.1 Backend (Rust)
- **Hosting**: Railway.app or Fly.io (easy Rust deploys)
- **Database**: Neon.tech (serverless Postgres, free tier)
- **Redis**: Upstash (serverless Redis, free tier)
- **Domain**: api.dride.app via Cloudflare DNS

### 12.2 Dockerfile (Rust Backend)

```dockerfile
FROM rust:1.82-slim AS builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/dride-backend /usr/local/bin/
EXPOSE 8080
CMD ["dride-backend"]
```

### 12.3 Solana Program
- Deploy to Devnet first via `anchor deploy --provider.cluster devnet`
- Audit before Mainnet deploy (Sec3, OtterSec, or Neodyme)
- Program upgrade authority: multisig via Squads Protocol

### 12.4 iOS App
- TestFlight for beta (up to 10,000 testers)
- App Store submission requires:
  - Privacy policy URL
  - Apple Developer Program ($99/year)
  - App Review guidelines compliance
  - Export compliance (encryption declaration for Solana crypto)

---

## 13. Testing Strategy

### 13.1 Backend (Rust)
```
tests/
├── api/
│   ├── auth_test.rs         # OTP flow, JWT validation
│   ├── users_test.rs        # CRUD operations
│   ├── rides_test.rs        # full ride lifecycle
│   ├── ratings_test.rs      # rating creation
│   └── wallet_test.rs       # balance queries
└── solana/
    └── escrow_test.rs       # program integration tests
```

- **Unit tests**: `cargo test` for business logic (pricing, matching, status transitions)
- **Integration tests**: test containers (PostgreSQL + Redis) for API endpoint testing
- **Solana tests**: `anchor test` with local validator

### 13.2 iOS
- **Unit tests (XCTest)**: ViewModels, services, models
- **UI tests (XCUITest)**: critical flows (auth, request ride, complete ride)
- **Preview testing**: SwiftUI previews for every view with mock data

### 13.3 Smart Contract
- **Anchor tests** (TypeScript): `anchor test`
- Test cases:
  - Happy path: create → accept → complete → funds split correctly
  - Cancel before accept → full refund
  - Cancel after accept → full refund (grace period)
  - Double complete attempt → error
  - Wrong authority → error
  - Insufficient funds → error

---

## 14. Implementation Roadmap

### Phase 1: Foundation (Semanas 1-2)
**Goal:** Backend running + database + auth flow

- [ ] Init Rust project with Axum boilerplate
- [ ] PostgreSQL schema migrations (all 6 tables)
- [ ] `POST /auth/otp/request` + `POST /auth/otp/verify`
- [ ] JWT middleware
- [ ] `GET/PATCH/DELETE /users/me`
- [ ] Basic error handling + logging
- [ ] Docker Compose (Postgres + Redis + backend)
- [ ] Integration tests for auth + users

### Phase 2: Smart Contract (Semanas 2-3)
**Goal:** Escrow program deployed on Devnet

- [ ] Anchor project setup
- [ ] `RideEscrow` account struct
- [ ] `create_ride` instruction (deposit SOL)
- [ ] `complete_ride` instruction (90/10 split)
- [ ] `cancel_ride` instruction (full refund)
- [ ] Anchor tests for all paths
- [ ] Deploy to Devnet

### Phase 3: Ride CRUD + Matching (Semanas 3-4)
**Goal:** Full ride lifecycle through API

- [ ] `POST /rides/estimate` (pricing engine)
- [ ] `POST /rides` (create ride + escrow PDA)
- [ ] `POST /rides/:id/deposit-confirm`
- [ ] `GET /rides/available` (geo query for drivers)
- [ ] `POST /rides/:id/accept`
- [ ] `POST /rides/:id/start`
- [ ] `POST /rides/:id/complete` (trigger escrow release)
- [ ] `POST /rides/:id/cancel` (trigger escrow refund)

- [ ] `POST /rides/:id/rate`
- [ ] `GET /rides/history`
- [ ] Ride status machine validation
- [ ] SOL/BRL rate caching (Redis + CoinGecko)
- [ ] Integration tests for full ride lifecycle

### Phase 4: WebSocket + Real-time (Semana 4-5)
**Goal:** Live location tracking between driver and passenger

- [ ] WebSocket upgrade handler
- [ ] Connection hub (register/unregister by user_id)
- [ ] `location_update` from driver → store in ride_locations
- [ ] `driver_location` broadcast to passenger
- [ ] `ride_requested` broadcast to nearby drivers
- [ ] `ride_accepted`, `ride_started`, `ride_completed` events
- [ ] `deposit_confirmed`, `escrow_released` events
- [ ] Reconnection handling
- [ ] Background jobs: expire stale rides, sync balances

### Phase 5: iOS App — Core (Semanas 5-7)
**Goal:** Working app with auth, map, ride request

- [ ] Xcode project setup (Swift 6, iOS 17+ target)
- [ ] APIClient with async/await + JWT interceptor
- [ ] WebSocketManager with auto-reconnect
- [ ] WalletManager (Keychain key generation + storage)
- [ ] LocationService (CLLocationManager wrapper)
- [ ] Auth flow: PhoneInputView → OTPVerifyView
- [ ] PassengerHomeView: map + user location
- [ ] DestinationSearchView: MKLocalSearch autocomplete
- [ ] RideConfirmView: show fare, sign + send deposit tx

### Phase 6: iOS App — Full Flow (Semanas 7-9)
**Goal:** Complete ride flow end-to-end

- [ ] WaitingForDriverView: searching animation + cancel
- [ ] RideActiveView: live driver tracking on map
- [ ] RideCompletedView: receipt + rating
- [ ] Driver mode: DriverHomeView with online toggle
- [ ] RideRequestCard: incoming ride popup with accept/decline
- [ ] NavigationView: route to pickup → route to dropoff
- [ ] DriverRideCompleteView: earnings summary
- [ ] WalletView: balance + transaction history
- [ ] ProfileView: edit name, photo, vehicle info
- [ ] RideHistoryView: paginated list
- [ ] Push notifications (APNs) for ride events

### Phase 7: Polish + Launch (Semanas 9-12)
**Goal:** TestFlight beta → App Store

- [ ] Error handling polish (offline states, retry logic)
- [ ] Loading states and skeleton views
- [ ] Haptic feedback on key actions
- [ ] App icon and launch screen
- [ ] Onboarding tutorial (3 screens)
- [ ] Privacy policy + Terms of Service pages
- [ ] TestFlight internal testing (10 people)
- [ ] TestFlight public beta (Porto Seguro drivers)
- [ ] Smart contract audit (at minimum self-audit checklist)
- [ ] Deploy backend to Railway/Fly.io production
- [ ] Switch Solana to Mainnet
- [ ] App Store submission

---

## 15. Future Roadmap (Post-MVP)

### v0.2
- [ ] PIX on-ramp: passenger pays PIX → receives SOL in wallet
- [ ] Surge pricing (supply/demand algorithm)
- [ ] Dispute resolution with on-chain arbitration
- [ ] Partial refund on cancel after ride start
- [ ] Driver documents verification (CNH upload)
- [ ] In-app chat between driver and passenger

### v0.3
- [ ] Android app (Kotlin / Jetpack Compose)
- [ ] Multisig program upgrade authority (Squads)
- [ ] DAO governance: token holders vote on fee changes
- [ ] Ride pooling (shared rides, split fare)
- [ ] Driver staking: stake SOL to get priority matching
- [ ] Expand to more cities

### v1.0
- [ ] Protocol token (dRIDE) for governance + fee discounts
- [ ] Multi-chain support (Base L2 for cheaper EVM option)
- [ ] Insurance fund (smart contract pool for accident coverage)
- [ ] Open-source protocol: any city can fork and run their own

---

*End of specification. This document is the single source of truth for the dRide project.*
*Feed this to Claude Code with: `claude "read SPEC.md and start Phase 1"`*
