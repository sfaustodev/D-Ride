# dRide — Decentralized Ride-Sharing on Solana

> **Drivers keep 90%. No algorithm. No middleman. On-chain.**

[![Live Site](https://img.shields.io/badge/Live%20Site-dride.app-6C5CE7?style=for-the-badge)](https://landing-sage-seven-93.vercel.app)
[![Solana](https://img.shields.io/badge/Solana-Mainnet-9945FF?style=for-the-badge&logo=solana)](https://solana.com)
[![Next.js](https://img.shields.io/badge/Next.js-15-black?style=for-the-badge&logo=next.js)](https://nextjs.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-00B894?style=for-the-badge)](LICENSE)

---

> ⭐ **If you came through the Solana Public Grants Program — please star this repo to show your support!** It helps us a lot and signals community backing to the grant evaluators. Thank you!

---

## Preview

```
┌─────────────────────────────────────────────────────────────────┐
│  dRide                                    [Connect Wallet]  EN/PT│
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│         Motoristas merecem mais.                                  │
│         The protocol that pays                                    │
│         drivers 90% — on-chain,                                   │
│         every ride, no exceptions.                                │
│                                                                   │
│         [Join Presale]   [Read Whitepaper]                        │
│                                                                   │
│    📊 1,200+ Wallets  |  $47K Raised  |  <1s Settlement          │
├─────────────────────────────────────────────────────────────────┤
│  THE PROBLEM                                                      │
│  Uber takes ~42% of every ride.                                   │
│  A driver earning R$20 gets only R$12.                            │
│                                                                   │
│  THE SOLUTION                                                      │
│  A Solana smart contract splits every payment:                    │
│  90% → Driver  |  10% → Protocol  |  0% → middlemen              │
├─────────────────────────────────────────────────────────────────┤
│  Comparison    │   Uber      │   dRide                            │
│  Platform fee  │   ~42%      │   10%                              │
│  Driver gets   │   R$12      │   R$18                             │
│  Tx cost       │   R$0.50    │   R$0.02                           │
│  Fee source    │   Secret AI │   On-chain formula                 │
│  Payment time  │   1 week    │   Instant                          │
├─────────────────────────────────────────────────────────────────┤
│  HOW IT WORKS                                                     │
│  1. Passenger requests ride + funds go to escrow PDA             │
│  2. Driver accepts → rides happen                                 │
│  3. Smart contract releases: 90% driver, 10% protocol            │
│  4. All on Solana — settled in < 1 second                         │
├─────────────────────────────────────────────────────────────────┤
│  $DRIDE TOKEN PRESALE                                             │
│  Price: $0.005  |  Next round: $0.008  |  Hard cap: 1M SOL       │
│  ████████████░░░░░░░░░░░░  Progress                               │
│  [Connect Phantom Wallet to Participate]                          │
├─────────────────────────────────────────────────────────────────┤
│  TOKENOMICS (1 Billion $DRIDE)                                    │
│  30% Presale & Public Sale                                        │
│  25% Ecosystem & Driver Rewards                                   │
│  20% Team (4yr vest, 1yr cliff)                                   │
│  15% Liquidity (Raydium/Orca, locked)                             │
│  10% Treasury (Squads multisig)                                   │
├─────────────────────────────────────────────────────────────────┤
│  ROADMAP                                                          │
│  ✅ Q2 2026 — Backend (Rust/Axum) + Solana Escrow + WebSocket    │
│  ◻  Q3 2026 — iOS App (SwiftUI) + TestFlight Beta                │
│  ◻  Q4 2026 — App Store + Mainnet + TGE                          │
│  ◻  Q1 2027 — PIX on-ramp + Android + New Cities                 │
│  ◻  Q2 2027 — DAO Governance + Open Source + Multi-chain         │
├─────────────────────────────────────────────────────────────────┤
│  TEAM                                                             │
│  Juan Fausto — Founder & Dev                                      │
│  [LinkedIn]  [GitHub]                                             │
│                                                                   │
│  CTO · Advisor — Future                                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| Frontend | Next.js 15 + TypeScript |
| Styling | Tailwind CSS 4 + Framer Motion |
| Wallet | @solana/wallet-adapter-react (Phantom, Solflare, Backpack) |
| Smart Contract | Anchor / Solana — Escrow PDA |
| i18n | next-intl (EN / PT-BR) |
| Hosting | Vercel |

---

## Getting Started

```bash
# Clone the repo
git clone https://github.com/sfaustodev/D-Ride.git
cd D-Ride/landing

# Install dependencies
pnpm install

# Set environment variables
cp .env.example .env.local
# Fill in: NEXT_PUBLIC_PRESALE_WALLET, NEXT_PUBLIC_SOLANA_NETWORK, etc.

# Start dev server
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000).

---

## Project Structure

```
landing/
├── src/
│   ├── app/[locale]/         # Internationalized routes (en, pt)
│   ├── components/
│   │   ├── sections/         # Hero, Problem, HowItWorks, Presale,
│   │   │                     # Tokenomics, Roadmap, Team, FAQ
│   │   ├── ui/               # Button, Card, Badge, GlowCard...
│   │   └── layout/           # Navbar, Footer
│   ├── hooks/
│   │   └── usePresale.ts     # Wallet + presale logic
│   └── lib/
│       └── constants.ts      # Addresses, prices, protocol config
└── messages/
    ├── en.json               # English strings
    └── pt.json               # Portuguese strings
```

---

## Protocol Constants

| Constant | Value |
|---|---|
| Protocol fee | 10% |
| Driver share | 90% |
| Presale price | $0.005 / $DRIDE |
| Next round price | $0.008 / $DRIDE |
| Tx cost on Solana | ~$0.02 |
| Settlement time | < 1 second |
| Escrow PDA seeds | `["ride", ride_uuid_bytes]` |

---

## Contact

**Juan Fausto** — Founder & Dev

- LinkedIn: [linkedin.com/in/juan-fausto-07b311193](https://www.linkedin.com/in/juan-fausto-07b311193/)
- GitHub: [github.com/sfaustodev](https://github.com/sfaustodev)
- Email: [team@dride.app](mailto:team@dride.app)

---

## License

MIT — see [LICENSE](LICENSE) for details.
