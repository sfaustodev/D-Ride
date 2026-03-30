# dRide — Decentralized Ride-Sharing on Solana

<div align="center">

**Peer-to-peer rides. Drivers keep 90%. Zero corporate middlemen.**

[![Rust](https://img.shields.io/badge/Backend-Rust%20%2F%20Axum-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Swift](https://img.shields.io/badge/iOS-Swift%206%20%2F%20SwiftUI-blue?style=flat-square&logo=swift)](https://developer.apple.com/swift/)
[![Solana](https://img.shields.io/badge/Chain-Solana%20%2F%20Anchor-9945FF?style=flat-square&logo=solana)](https://solana.com/)
[![Next.js](https://img.shields.io/badge/Landing-Next.js%2015-black?style=flat-square&logo=next.js)](https://nextjs.org/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

</div>

---

## What is dRide?

dRide replaces the Uber model with a transparent, on-chain protocol. When a passenger books a ride, their payment goes into a **Solana escrow smart contract** — not a corporate wallet. When the ride completes, the driver receives **90% of the fare instantly**. The protocol takes a flat **10% fee**, auditable by anyone on-chain.

No hidden fees. No surge pricing. No corporation between driver and passenger.

---

## Architecture

```
┌─────────────────────────────────────┐
│           iOS App (Swift)           │
│  SwiftUI + MapKit + WalletManager   │
└──────────────┬──────────────────────┘
               │ HTTPS + WSS
┌──────────────▼──────────────────────┐
│        Rust Backend (Axum)          │
│  REST API + WebSocket Hub + Jobs    │
│  PostgreSQL + Redis                 │
└──────────┬─────────┬────────────────┘
           │         │
┌──────────▼───┐ ┌───▼──────────────────┐
│  PostgreSQL  │ │  Solana RPC           │
│  (rides,     │ │  (Helius / QuickNode) │
│   users, etc)│ │                       │
└──────────────┘ └───────┬──────────────┘
                         │
                ┌────────▼──────────┐
                │  Anchor Program   │
                │  (Escrow + Ride)  │
                └───────────────────┘
```

- The iOS app talks to the Rust backend via REST + WebSocket
- The backend orchestrates the full ride lifecycle and triggers Solana transactions
- The smart contract holds funds in escrow until the ride completes
- **Private keys never leave the iOS device**

---

## Repository Structure

```
dride/
├── dride-ios/              # iOS app — Swift 6 / SwiftUI / MapKit
├── dride-backend/          # API server — Rust / Axum / PostgreSQL / Redis
├── dride-escrow/           # Ride escrow — Solana / Anchor smart contract
├── dride-presale/          # Token presale — Solana / Anchor program
├── dride-token/            # DRIDE SPL token — Solana / Anchor program
├── dride-leptos-landing/   # Alternative landing — Rust / Leptos / WASM
├── landing/                # Landing page + presale UI — Next.js 15
└── SPEC.md                 # Complete technical specification (read this first)
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| iOS App | Swift 6, SwiftUI, MapKit, iOS 17+, `@Observable` |
| Backend | Rust, Axum 0.8, SQLx, PostgreSQL, Redis, Tokio |
| Smart Contracts | Solana, Anchor framework, Rust |
| Landing Page | Next.js 15, React 19, Tailwind CSS 4, Framer Motion |
| Wallet | @solana/wallet-adapter (Phantom, Solflare, Backpack) |
| Auth | JWT (7-day expiry), SMS OTP via Redis |
| Payments | SOL lamports via on-chain PDA escrow |
| i18n | next-intl (English + Portuguese) |

---

## Smart Contracts

### Escrow Program — `dride-escrow/`

Manages payment lifecycle for each ride. Funds are locked in a **PDA** derived from the ride UUID and released only on completion or cancelled on timeout.

```
PDA seeds:  ["ride", ride_uuid_bytes]
Fee:        1000 bps (10%)
Timeout:    300s (5 min) for driver match
Amounts:    u64 lamports — never floating point
```

**Instructions:**
| Instruction | Description |
|---|---|
| `initialize_ride` | Lock passenger funds in escrow PDA |
| `complete_ride` | Release 90% to driver, 10% to protocol |
| `cancel_ride` | Refund passenger (pre-match or timeout) |

### Presale Program — `dride-presale/`

On-chain token presale with hard cap enforcement, per-wallet limits, and emergency pause.

```
Hard Cap:     1,000,000 SOL
Rate:         1 SOL = 200 DRIDE
Min Purchase: 0.1 SOL
Max/Wallet:   1,000 SOL
```

### DRIDE Token — `dride-token/`

SPL token with multisig mint authority and no freeze authority.

```
Symbol:    DRIDE
Decimals:  9
Supply:    1,000,000,000 (1 billion)
Standard:  SPL Token (Solana Program Library)
```

---

## Tokenomics

| Allocation | % | Amount |
|---|---|---|
| Presale & Public Sale | 30% | 300M DRIDE |
| Ecosystem & Rewards | 25% | 250M DRIDE |
| Team & Advisors (vested) | 20% | 200M DRIDE |
| Liquidity Pool (locked) | 15% | 150M DRIDE |
| Treasury / Reserve | 10% | 100M DRIDE |

---

## Backend — `dride-backend/`

Rust/Axum REST API and WebSocket server.

**Key endpoints:**

| Method | Path | Description |
|---|---|---|
| POST | `/auth/request-otp` | Send SMS OTP |
| POST | `/auth/verify-otp` | Verify OTP, return JWT |
| POST | `/rides` | Request a new ride |
| POST | `/rides/:id/accept` | Driver accepts ride |
| POST | `/rides/:id/complete` | Complete ride + trigger escrow release |
| WS | `/ws` | Live driver location + ride status |

**Stack details:**
- `sqlx` with compile-time checked queries
- `thiserror` for typed errors, `anyhow` for ad-hoc
- `tracing` for structured logging
- `argon2` for password hashing
- `validator` for request body validation
- `tower-http` for CORS and tracing middleware

---

## iOS App — `dride-ios/`

SwiftUI app for iOS 17+. Single app with passenger/driver role toggle.

**Key screens:**
- Onboarding (phone auth via SMS OTP)
- Passenger map (request ride, track driver)
- Driver map (accept requests, navigate)
- Ride history and ratings
- Wallet (embedded Keychain-backed SOL wallet)

**Architecture:**
- `@Observable` ViewModels on `@MainActor`
- `actor` for thread-safe services (NetworkService, WalletManager)
- `async/await` throughout — no Combine
- `LocalizedStringKey` for all user-facing strings

---

## Landing Page — `landing/`

Marketing site + token presale interface.

```bash
cd landing
pnpm install
pnpm dev        # http://localhost:3000
pnpm build      # production build
```

**Features:**
- Animated scroll sections (Framer Motion)
- Token presale widget (live countdown, progress bar)
- Solana wallet connection (Phantom, Solflare, Backpack)
- Bilingual: English + Portuguese (next-intl)
- Dark glass morphism design

---

## Development Setup

### Prerequisites

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Solana CLI + Anchor
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest && avm use latest

# Node.js (pnpm)
npm install -g pnpm

# iOS: Xcode 15+ required
```

### Backend

```bash
cd dride-backend
cp .env.example .env               # configure DATABASE_URL, REDIS_URL, etc.
docker compose up -d               # start PostgreSQL + Redis
sqlx migrate run                   # apply migrations
cargo run                          # start on :3000
cargo test                         # run all tests
```

### Smart Contracts

```bash
cd dride-escrow
anchor build                       # compile program
anchor test                        # run tests with local validator
anchor deploy --provider.cluster devnet
```

### iOS App

```bash
cd dride-ios
open dRide.xcodeproj               # open in Xcode
# Cmd+R to run on simulator
# Cmd+U to run tests
```

### Landing Page

```bash
cd landing
pnpm install
cp .env.local.example .env.local   # set NEXT_PUBLIC_* vars
pnpm dev                           # http://localhost:3000
```

---

## Quick Reference

| Constant | Value |
|---|---|
| Protocol fee | 1000 bps (10%) |
| Driver receives | 90% |
| Escrow PDA seeds | `["ride", ride_uuid_bytes]` |
| Driver match timeout | 300s (5 min) |
| Location update interval | 3s (during active ride) |
| JWT expiry | 7 days |
| OTP expiry | 5 minutes |
| Driver search radius | 5 km |
| DRIDE decimals | 9 |
| All monetary values | `u64` lamports (Rust/contract), `Int64` (iOS) |

---

## Roadmap

- [x] Phase 1 — Backend foundation (auth, users, rides API)
- [x] Phase 2 — WebSocket live tracking
- [x] Phase 3 — Solana escrow smart contract
- [x] Phase 4 — DRIDE token + presale program
- [ ] Phase 5 — iOS app (SwiftUI)
- [ ] Phase 6 — Devnet integration testing
- [ ] Phase 7 — Mainnet launch (Porto Seguro, BA)
- [ ] Phase 8 — Multi-city expansion

---

## Contact

**Fausto** — Founder & Developer
- Email: sfaustodev@gmail.com
- Project: [dride.io](https://dride.io)

---

<div align="center">
<sub>Built with Rust, Swift, and Solana. Drivers keep 90%.</sub>
</div>
