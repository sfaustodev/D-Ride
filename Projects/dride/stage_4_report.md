# dRide — Phase 4 Report: WebSocket + Real-time

**Date:** 2026-03-23
**Branch:** `main`
**Tests:** 14/14 passing

---

## Commits (Phase 4)

| Hash | Message |
|------|---------|
| `49534cf` | add WebSocket hub, handler, and message types |
| `670e12e` | add ride_location and ride_event models |
| `6fca84a` | add expire_rides background job (5 min timeout) |
| `4654a15` | add real-time WS events to ride accept/start/complete/cancel |
| `e1d3ad1` | wire WS route, Hub in AppState, spawn jobs, add futures-util |

---

## What Was Built

### WebSocket Module (`src/ws/`)

**`hub.rs`** — Connection Registry
- `Hub` struct with `Arc<RwLock<HashMap<Uuid, ConnectedUser>>>` for thread-safe access
- `register(user_id, tx)` — add user connection
- `unregister(user_id)` — remove on disconnect
- `update_location(user_id, lat, lng)` — track driver position in memory
- `send_to_user(user_id, msg)` — direct message to one user
- `send_to_users(user_ids, msg)` — direct message to multiple users
- `broadcast_nearby(lat, lng, radius_km, msg, exclude)` — geo-broadcast using haversine distance
- `is_connected(user_id)` — check if user is online

**`handler.rs`** — WebSocket Upgrade + Message Processing
- Endpoint: `GET /v1/ws?token=JWT`
- JWT validation before upgrade via `verify_token`
- Incoming messages:
  - `location_update` → updates hub location, stores in `ride_locations` DB, broadcasts `driver_location` to passenger
  - `ping` → keepalive (no-op)
- On disconnect: unregisters from hub, aborts send task

**`messages.rs`** — Typed Message Envelope
- `WsMessage { type, payload }` — JSON envelope for all WS messages
- Client → Server types:
  - `LocationUpdate { lat, lng, heading, speed_kmh }`
- Server → Client types:
  - `RideRequestedEvent { ride_id, pickup_address, dropoff_address, distance_km, fare_brl, pickup_distance_km }`
  - `RideAcceptedEvent { ride_id, driver_name, driver_rating }`
  - `RideCancelledEvent { ride_id, cancelled_by }`
  - `RideStartedEvent { ride_id, started_at }`
  - `RideCompletedEvent { ride_id, release_tx_sig }`
  - `DriverLocationEvent { ride_id, lat, lng, heading, eta_min }`
  - `DepositConfirmedEvent { ride_id, escrow_pubkey }`
  - `EscrowReleasedEvent { ride_id, tx_sig, driver_amount }`

### Real-time Event Integration (in ride handlers)

| Handler | Event Broadcast | Recipients |
|---------|----------------|------------|
| `accept` | `ride_accepted` | Passenger |
| `start` | `ride_started` | Passenger |
| `complete` | `ride_completed` | Passenger + Driver |
| `cancel` | `ride_cancelled` | Passenger + Driver |
| WS `location_update` | `driver_location` | Passenger |

All ride state changes also create a `ride_event` record for event sourcing.

### Models Added

**`src/models/ride_location.rs`**
- `RideLocation` struct mapping to `ride_locations` table
- `create(pool, ride_id, user_id, lat, lng, heading, speed_kmh)` — insert GPS point

**`src/models/ride_event.rs`**
- `RideEvent` struct mapping to `ride_events` table
- `create(pool, ride_id, event_type, actor_id, metadata)` — insert event record

### Background Jobs (`src/jobs/`)

**`expire_rides.rs`**
- Runs every 30 seconds
- Expires rides with status `requested` older than 5 minutes (300s)
- Sets status to `expired` via SQL UPDATE
- Logs count of expired rides when > 0

**`mod.rs`**
- `spawn_jobs(state)` — spawns all background tasks on tokio runtime

### Infrastructure Changes

**`src/lib.rs`**
- Added `pub mod jobs`, `pub mod ws`
- Added `hub: Hub` field to `AppState`

**`src/main.rs`**
- Initializes `Hub::new()` in AppState
- Calls `jobs::spawn_jobs(state.clone())` before server start
- Mounts WS route at `/v1/ws`

**`Cargo.toml`**
- Added `futures-util = "0.3"` for WebSocket stream splitting

**`tests/helpers.rs`**
- Added `hub: Hub::new()` to test AppState

---

## Tests (14 total, all passing)

### Auth + Users (7)

| Test | Status |
|------|--------|
| `test_request_otp_success` | PASS |
| `test_request_otp_invalid_phone` | PASS |
| `test_verify_otp_creates_user` | PASS |
| `test_get_me_unauthorized` | PASS |
| `test_get_me_success` | PASS |
| `test_update_me` | PASS |
| `test_delete_me` | PASS |

### Rides + Ratings (7)

| Test | Status |
|------|--------|
| `test_estimate` | PASS |
| `test_create_ride` | PASS |
| `test_full_ride_lifecycle` | PASS |
| `test_cancel_ride_before_accept` | PASS |
| `test_ride_history` | PASS |
| `test_cannot_complete_non_active_ride` | PASS |
| `test_duplicate_rating_rejected` | PASS |

---

## Files Added/Modified

### New Files (7)
```
src/ws/mod.rs               — WebSocket module
src/ws/hub.rs               — Connection registry + geo-broadcast
src/ws/handler.rs           — WS upgrade + message dispatch
src/ws/messages.rs          — Typed WS message enums
src/models/ride_location.rs — GPS tracking model
src/models/ride_event.rs    — Event sourcing model
src/jobs/mod.rs             — Background job runner
src/jobs/expire_rides.rs    — Stale ride expiration job
```

### Modified Files (5)
```
src/lib.rs                  — Added ws, jobs modules + Hub in AppState
src/main.rs                 — WS route, Hub init, job spawning
src/models/mod.rs           — Added ride_location, ride_event modules
src/handlers/rides.rs       — Added WS event broadcasts + ride_event logging
Cargo.toml                  — Added futures-util
tests/helpers.rs            — Added Hub to test AppState
```

---

## WebSocket Protocol Summary

```
Client connects: GET /v1/ws?token=<JWT>
Server validates JWT, upgrades to WebSocket

Client → Server:
  { "type": "location_update", "payload": { "lat": -16.44, "lng": -39.06, "heading": 180.0, "speed_kmh": 30.0 } }
  { "type": "ping", "payload": {} }

Server → Client:
  { "type": "ride_accepted", "payload": { "ride_id": "uuid", "driver_name": "João", "driver_rating": 4.8 } }
  { "type": "ride_started", "payload": { "ride_id": "uuid", "started_at": "2026-03-23T..." } }
  { "type": "ride_completed", "payload": { "ride_id": "uuid", "release_tx_sig": "5UKg3..." } }
  { "type": "ride_cancelled", "payload": { "ride_id": "uuid", "cancelled_by": "passenger" } }
  { "type": "driver_location", "payload": { "ride_id": "uuid", "lat": -16.44, "lng": -39.06, "heading": 180.0, "eta_min": 3 } }
```

---

## Cumulative Progress

```
Phase 1  [====================] 100%  Backend foundation
Phase 2  [=================== ] 95%   Smart contract (missing devnet deploy)
Phase 3  [====================] 100%  Ride CRUD + matching
Phase 4  [====================] 100%  WebSocket + real-time
Phase 5  [                    ] 0%    iOS app core
Phase 6  [                    ] 0%    iOS app full flow
Phase 7  [                    ] 0%    Polish + launch
```

---

## TODOs (for future phases)

- [ ] On-chain deposit verification via Solana RPC in `deposit_confirm`
- [ ] On-chain escrow release via Solana RPC in `complete`
- [ ] On-chain escrow refund via Solana RPC in `cancel`
- [ ] ETA calculation in `driver_location` events
- [ ] `ride_requested` geo-broadcast to nearby connected drivers
- [ ] Deploy escrow program to Devnet
- [ ] WebSocket reconnection handling on iOS side
