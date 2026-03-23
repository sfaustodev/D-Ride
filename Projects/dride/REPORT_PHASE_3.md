# dRide — Project Status Report

**Date:** 2026-03-22
**Repository:** [github.com/sfaustodev/D-Ride](https://github.com/sfaustodev/D-Ride)
**Branch:** `main`

---

## Commits

| Hash | Message | Files Changed |
|------|---------|---------------|
| `afe9340` | Phase 1: Backend foundation — Axum server, auth, users CRUD | 35 files, +5789 |
| `7acfda4` | Phase 2: Solana escrow smart contract (Anchor) | 19 files, +3430 |

---

## Phase Status

### Phase 1: Foundation — COMPLETE

**Stack:** Rust / Axum / sqlx / PostgreSQL / Redis

| Component | Status | Files |
|-----------|--------|-------|
| Axum server + config | Done | `main.rs`, `config.rs`, `lib.rs` |
| Error handling | Done | `error.rs` (thiserror, 7 variants) |
| Database pool | Done | `db/pool.rs` |
| JWT auth (HS256, 7d) | Done | `auth/jwt.rs` |
| Auth middleware | Done | `auth/middleware.rs` |
| OTP flow (Redis-backed) | Done | `auth/otp.rs`, `handlers/auth.rs` |
| Users CRUD | Done | `handlers/users.rs`, `models/user.rs` |
| Wallet model | Done | `models/wallet.rs` |
| Docker Compose | Done | `docker-compose.yml` (Postgres 16 + Redis 7) |

**Migrations (6):**

| # | Migration | Purpose |
|---|-----------|---------|
| 001 | `create_users` | Users table + pgcrypto + earthdistance extensions |
| 002 | `create_rides` | Rides table (full lifecycle fields) |
| 003 | `create_ride_events` | Event sourcing for ride state changes |
| 004 | `create_ride_locations` | GPS tracking points during rides |
| 005 | `create_ratings` | Post-ride ratings (1-5 stars) |
| 006 | `create_wallets` | Solana wallet pubkeys per user |

**Backend Tests (7):**

| Test | Status | Description |
|------|--------|-------------|
| `test_request_otp_success` | PASS | POST /auth/otp/request with valid phone |
| `test_request_otp_invalid_phone` | PASS | Rejects phone without `+` prefix |
| `test_verify_otp_creates_user` | PASS | Full OTP verify flow, creates user + JWT |
| `test_get_me_unauthorized` | PASS | GET /users/me without token returns 401 |
| `test_get_me_success` | PASS | GET /users/me with valid JWT |
| `test_update_me` | PASS | PATCH /users/me updates name + role |
| `test_delete_me` | PASS | DELETE /users/me soft-deletes user |

> Tests require Docker (Postgres + Redis) running.

---

### Phase 2: Smart Contract — COMPLETE

**Stack:** Solana / Anchor 0.32.1 / Rust

**Program ID:** `2fhZ4fGn3NoU64UCbPxKEYEakPXXBTHJsHCw73QV7APx`

| Component | Status | File |
|-----------|--------|------|
| RideEscrow PDA account | Done | `state/ride_escrow.rs` |
| EscrowStatus enum | Done | Created, Active, Completed, Cancelled, Disputed |
| `create_ride` instruction | Done | `instructions/create_ride.rs` |
| `accept_ride` instruction | Done | `instructions/accept_ride.rs` |
| `complete_ride` instruction | Done | `instructions/complete_ride.rs` |
| `cancel_ride` instruction | Done | `instructions/cancel_ride.rs` |
| Custom errors | Done | `errors.rs` (8 variants) |

**PDA Design:**
- Seeds: `["ride", ride_uuid_bytes]`
- Account size: 140 bytes (8 discriminator + 132 data)
- Protocol fee: configurable in basis points (default 1000 = 10%)

**Escrow Flow:**
```
create_ride (passenger deposits SOL)
    -> accept_ride (authority assigns driver)
        -> complete_ride (90% driver + 10% protocol)
        -> cancel_ride (100% refund to passenger)
    -> cancel_ride (100% refund to passenger)
```

**Smart Contract Tests (7):**

| Test | Status | Description |
|------|--------|-------------|
| `creates a ride escrow with deposit` | PASS | Deposits SOL, verifies all account fields |
| `rejects zero amount` | PASS | InvalidAmount error on 0 lamports |
| `rejects fee too high (>50%)` | PASS | FeeTooHigh error on 5001 bps |
| `happy path: create -> accept -> complete` | PASS | Full flow, verifies 90/10 split amounts |
| `cancel before accept -> full refund` | PASS | Refund from Created status |
| `cancel after accept -> full refund` | PASS | Refund from Active status |
| `double complete attempt -> error` | PASS | InvalidStatus on second complete |

**Not yet done for Phase 2:**
- Devnet deployment (program compiled but not deployed)

---

### Phase 3: Ride CRUD + Matching — NOT STARTED

**Planned endpoints:**
- `POST /rides/estimate` — pricing engine
- `POST /rides` — create ride + escrow PDA
- `POST /rides/:id/deposit-confirm`
- `GET /rides/available` — geo query for nearby drivers
- `POST /rides/:id/accept`
- `POST /rides/:id/start`
- `POST /rides/:id/complete` — trigger escrow release
- `POST /rides/:id/cancel` — trigger escrow refund
- `POST /rides/:id/rate`
- `GET /rides/history`
- SOL/BRL rate caching via Redis + CoinGecko

### Phase 4: WebSocket + Real-time — NOT STARTED
### Phase 5: iOS App Core — NOT STARTED
### Phase 6: iOS App Full Flow — NOT STARTED
### Phase 7: Polish + Launch — NOT STARTED

---

## Full Roadmap

```
Phase 1  [====================] 100%  Backend foundation
Phase 2  [=================== ] 95%   Smart contract (missing devnet deploy)
Phase 3  [                    ] 0%    Ride CRUD + matching
Phase 4  [                    ] 0%    WebSocket + real-time
Phase 5  [                    ] 0%    iOS app core
Phase 6  [                    ] 0%    iOS app full flow
Phase 7  [                    ] 0%    Polish + launch
```

---

## Bug Fixes Applied (post Phase 2)

| Fix | File | Problem | Solution |
|-----|------|---------|----------|
| Docker ARM64 | `docker-compose.yml` | Containers crashed with `exec format error` on Apple Silicon | Added `platform: linux/arm64` to both services |
| Soft delete overflow | `src/models/user.rs` | `CONCAT('deleted_', uuid)` = 44 chars, exceeded `VARCHAR(20)` | Changed to `CONCAT('del_', LEFT(id::text, 14))` = 18 chars |
| Test idempotency | `tests/api/auth_test.rs` | Hardcoded phone `+5573999009999` caused `is_new_user=false` on re-runs | Randomized phone with `rand::random` |

---

## Known Issues

| Issue | Severity | Status |
|-------|----------|--------|
| Docker Desktop unstable on this machine | Medium | Reinstalled, needs verification |
| Smart contract not deployed to Devnet | Low | `anchor deploy` pending |
| Ambiguous glob re-export warning in escrow lib.rs | Trivial | Harmless, `handler` fn names collide |

---

## File Tree

```
dride/
├── CLAUDE.md
├── SPEC.md
├── REPORT_PHASE_3.md
├── .gitignore
├── dride-backend/
│   ├── Cargo.toml / Cargo.lock
│   ├── docker-compose.yml
│   ├── .env.example
│   ├── migrations/          (6 SQL files)
│   ├── src/
│   │   ├── main.rs          (server entrypoint)
│   │   ├── lib.rs           (AppState, re-exports)
│   │   ├── config.rs        (env-based config)
│   │   ├── error.rs         (AppError enum)
│   │   ├── auth/            (jwt, middleware, otp)
│   │   ├── db/              (pool setup)
│   │   ├── handlers/        (auth, users)
│   │   └── models/          (user, wallet)
│   └── tests/               (7 integration tests)
└── dride-escrow/
    ├── Anchor.toml
    ├── Cargo.toml / Cargo.lock
    ├── programs/dride-escrow/src/
    │   ├── lib.rs            (program entrypoint)
    │   ├── errors.rs         (8 custom errors)
    │   ├── instructions/     (create, accept, complete, cancel)
    │   └── state/            (RideEscrow account + EscrowStatus)
    └── tests/                (7 Anchor/Mocha tests)
```

---

## Next Steps

1. Start **Phase 3**: Ride CRUD + matching engine + Solana integration from backend
2. Deploy escrow program to Devnet
3. Implement SOL/BRL rate caching
