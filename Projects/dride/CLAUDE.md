# dRide — Claude Code Project Instructions

## What is this project?
A decentralized ride-sharing app (like Uber but peer-to-peer) where payments go through a Solana escrow smart contract. The protocol takes a transparent 10% fee. Drivers keep 90%.

## Tech Stack
- **iOS App**: Swift 6 / SwiftUI / MapKit / iOS 17+
- **Backend**: Rust / Axum / sqlx / PostgreSQL / Redis
- **Smart Contract**: Solana / Anchor framework
- **Payments**: SOL via on-chain escrow PDA

## Architecture
- iOS app talks to Rust backend via REST + WebSocket
- Backend orchestrates ride lifecycle and triggers Solana transactions
- Smart contract holds funds in escrow until ride completes
- Private keys never leave the iOS device

## Key Files
- `SPEC.md` — **READ THIS FIRST**. Complete technical specification.
- `dride-backend/` — Rust backend (Axum)
- `dride-ios/` — iOS app (SwiftUI)
- `programs/dride-escrow/` — Anchor smart contract

## Coding Conventions

### Rust Backend
- Use `thiserror` for error types, `anyhow` for ad-hoc errors
- All database queries via `sqlx` with compile-time checked queries where possible
- Handlers return `Result<Json<T>, AppError>`
- Use `tracing` for structured logging, never `println!`
- All request bodies validated with `validator` derive macros
- Prefer `async fn` everywhere, no blocking code on tokio runtime
- Tests go in `tests/` directory (integration) or inline `#[cfg(test)]` (unit)

### Swift / iOS
- SwiftUI only, no UIKit unless absolutely necessary
- `@Observable` for ViewModels (iOS 17+ Observation framework)
- `@MainActor` on all ViewModels that touch UI state
- `actor` for thread-safe services (NetworkService, WalletManager)
- `async/await` for all async work, no Combine unless needed for SwiftUI bindings
- Never force unwrap (`!`) — always use `guard let` or `if let`
- All strings user-facing must be in `Localizable.xcstrings`

### Solana / Anchor
- Use `require!()` macro for all validation checks
- Every instruction must validate all account constraints
- PDA seeds documented in comments above each `#[account]`
- Custom errors in `errors.rs` with clear messages
- All amounts in lamports (u64), never floating point

## Development Workflow

### Starting a new phase
1. Read the relevant phase in `SPEC.md` section 14
2. Create the directory structure first
3. Implement one file at a time, test as you go
4. Run tests before moving to next file

### Backend commands
```bash
cd dride-backend
cargo run                          # start dev server
cargo test                         # run all tests
cargo test api::rides_test         # run specific test module
sqlx migrate run                   # apply migrations
docker compose up -d postgres redis # start deps
```

### Smart Contract commands
```bash
cd programs/dride-escrow
anchor build                       # compile program
anchor test                        # run tests with local validator
anchor deploy --provider.cluster devnet  # deploy to devnet
```

### iOS commands
```bash
cd dride-ios
open dRide.xcodeproj              # open in Xcode
# Build: Cmd+B in Xcode
# Run on Simulator: Cmd+R in Xcode
# Run tests: Cmd+U in Xcode
```

## Important Rules
- **Never hardcode private keys** in source code
- **Never log sensitive data** (keys, OTP codes, JWTs)
- **All monetary values in lamports** (u64) on backend and contract
- **All monetary values in Int64** on iOS (fareLamports)
- **BRL display values are informational only** — SOL amount is canonical
- **Backend authority keypair** must be loaded from file/env, never committed
- **Rate SOL/BRL** is locked at ride request time and stored with the ride

## Quick Reference
- Escrow PDA seeds: `["ride", ride_uuid_bytes]`
- Protocol fee: 1000 bps (10%)
- Ride timeout: 300s (5 min) for driver match
- Location update interval: 3s (driver while in ride)
- JWT expiry: 7 days
- OTP expiry: 5 minutes
- Driver search radius: 5km

## How to Start
```
claude "Read SPEC.md completely, then implement Phase 1 step by step"
```
