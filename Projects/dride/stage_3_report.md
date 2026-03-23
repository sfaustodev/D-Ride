# dRide — Phase 3 Report: Ride CRUD + Matching

**Date:** 2026-03-23
**Branch:** `main`
**Tests:** 14/14 passing

---

## Commits (Phase 3)

| Hash | Message |
|------|---------|
| `c013de6` | add ride and rating models with full CRUD queries |
| `24d2145` | add pricing engine and SOL/BRL rate service |
| `862761d` | add ride lifecycle handlers and rating endpoint |
| `fe75d1f` | wire ride/rating routes into router, add rust_decimal dep |
| `a090d00` | add 7 integration tests for rides + ratings lifecycle |

---

## What Was Built

### Models

**`src/models/ride.rs`**
- `Ride` struct with full DB mapping (FromRow)
- `RideResponse` with f64 conversions for JSON serialization
- `RideStatus` enum with state machine validation:
  ```
  requested → accepted → deposit_pending → active → completing → completed
                 ↘ cancelled                     ↘ disputed
  requested → expired
  ```
- Request types: `CreateRideRequest`, `EstimateRequest`, `DepositConfirmRequest`, `CancelRequest`
- Queries: `find_by_id`, `create`, `update_status`, `set_driver`, `set_deposit_confirmed`, `start`, `complete`, `cancel`, `find_available` (geo query with earthdistance), `history` (paginated)

**`src/models/rating.rs`**
- `Rating` struct + `RatingResponse`
- `CreateRatingRequest` (score 1-5, optional comment)
- `create` — inserts rating with unique constraint on (ride_id, rater_id)
- `update_user_rating` — recalculates and caches rating_avg/rating_count on users table

### Services

**`src/services/pricing.rs`**
- Porto Seguro MVP fare constants:
  - Base fare: R$ 3.50
  - Per km: R$ 2.00
  - Per min: R$ 0.30
  - Minimum fare: R$ 6.00
  - Protocol fee: 1000 bps (10%)
- `calculate_fare(distance_km, duration_min, sol_brl_rate)` → `FareEstimate`
- `haversine_km(lat1, lng1, lat2, lng2)` — great-circle distance
- `estimate_duration_min(distance_km)` — 25 km/h city average heuristic

**`src/services/sol_rate.rs`**
- `get_sol_brl_rate(redis, coingecko_url)` — fetch from CoinGecko, cache 60s in Redis
- `get_sol_brl_rate_safe(...)` — with fallback to R$ 83.00
- Cache key: `sol_brl_rate`, TTL: 60s

### Handlers (10 new endpoints)

**`src/handlers/rides.rs`**

| Endpoint | Method | Handler | Description |
|----------|--------|---------|-------------|
| `/v1/rides/estimate` | POST | `estimate` | Fare preview from coordinates |
| `/v1/rides` | POST | `create_ride` | Create ride + escrow PDA reference |
| `/v1/rides/available` | GET | `available` | Nearby rides within 5km (geo query) |
| `/v1/rides/history` | GET | `history` | Paginated ride history by role |
| `/v1/rides/{id}` | GET | `get_ride` | Ride details (participant only) |
| `/v1/rides/{id}/deposit-confirm` | POST | `deposit_confirm` | Confirm on-chain deposit tx |
| `/v1/rides/{id}/accept` | POST | `accept` | Driver accepts ride |
| `/v1/rides/{id}/start` | POST | `start` | Driver starts trip |
| `/v1/rides/{id}/complete` | POST | `complete` | Complete + trigger escrow release |
| `/v1/rides/{id}/cancel` | POST | `cancel_ride` | Cancel + trigger escrow refund |

**`src/handlers/ratings.rs`**

| Endpoint | Method | Handler | Description |
|----------|--------|---------|-------------|
| `/v1/rides/{id}/rate` | POST | `rate_ride` | Rate counterpart (1-5 stars) |

### Router Changes

**`src/main.rs`** — All 10 ride routes + 1 rating route added to protected routes

**`src/lib.rs`** — Added `pub mod services`

**`Cargo.toml`** — Added `rust_decimal` with `serde-with-str` feature, `sqlx` gained `rust_decimal` feature

---

## Tests (14 total, all passing)

### Existing Tests (7) — Auth + Users

| Test | Status |
|------|--------|
| `test_request_otp_success` | PASS |
| `test_request_otp_invalid_phone` | PASS |
| `test_verify_otp_creates_user` | PASS |
| `test_get_me_unauthorized` | PASS |
| `test_get_me_success` | PASS |
| `test_update_me` | PASS |
| `test_delete_me` | PASS |

### New Tests (7) — Rides + Ratings

| Test | Status | Description |
|------|--------|-------------|
| `test_estimate` | PASS | POST /rides/estimate returns valid fare with distance, duration, lamports |
| `test_create_ride` | PASS | POST /rides returns 201 with ride object + escrow pubkey |
| `test_full_ride_lifecycle` | PASS | create → accept → start → complete → rate (5 stars) |
| `test_cancel_ride_before_accept` | PASS | Cancel from requested status, cancelled_by = passenger |
| `test_ride_history` | PASS | GET /rides/history returns at least 1 ride |
| `test_cannot_complete_non_active_ride` | PASS | Complete on accepted (not started) returns 409 Conflict |
| `test_duplicate_rating_rejected` | PASS | Second rating on same ride returns 400 |

---

## State Machine Validation

The `RideStatus` enum enforces valid transitions:

```
From            → To                    Allowed?
─────────────────────────────────────────────────
requested       → accepted              ✓
requested       → cancelled             ✓
requested       → expired               ✓
accepted        → deposit_pending       ✓
accepted        → cancelled             ✓
deposit_pending → requested             ✓ (deposit confirmed)
deposit_pending → cancelled             ✓
active          → completing            ✓
active          → cancelled             ✓
active          → disputed              ✓
completing      → completed             ✓
All others                              ✗ (returns 409)
```

---

## Files Added/Modified

### New Files (8)
```
src/models/ride.rs          — Ride model + queries
src/models/rating.rs        — Rating model + queries
src/services/mod.rs         — Services module
src/services/pricing.rs     — Fare calculation engine
src/services/sol_rate.rs    — SOL/BRL rate service
src/handlers/rides.rs       — 10 ride endpoints
src/handlers/ratings.rs     — Rating endpoint
tests/api/rides_test.rs     — 7 integration tests
```

### Modified Files (5)
```
src/models/mod.rs           — Added ride + rating modules
src/handlers/mod.rs         — Added rides + ratings modules
src/lib.rs                  — Added services module
src/main.rs                 — Wired 11 new routes
Cargo.toml                  — Added rust_decimal, sqlx rust_decimal feature
```

---

## TODOs (for future phases)

- [ ] On-chain deposit verification via Solana RPC in `deposit_confirm`
- [ ] On-chain escrow release via Solana RPC in `complete`
- [ ] On-chain escrow refund via Solana RPC in `cancel`
- [ ] Real PDA derivation using program ID + ride UUID (currently placeholder)
- [ ] Background job to expire stale rides (5 min timeout)
- [ ] Deploy escrow program to Devnet
