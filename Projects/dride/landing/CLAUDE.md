# dRide Landing Page — Claude Code Instructions

## What is this?
Landing page + token presale for dRide, a decentralized ride-sharing protocol on Solana.

## Tech Stack
- **Framework**: Next.js 15 (App Router) + TypeScript
- **Styling**: Tailwind CSS 4 + CSS variables for design tokens
- **Animations**: Framer Motion (scroll-triggered, staggered)
- **Wallet**: @solana/wallet-adapter-react (Phantom, Solflare, Backpack)
- **EVM**: ethers.js v6 (MetaMask, optional secondary)
- **UI primitives**: Radix UI (Dialog, Tabs, Accordion, Progress)
- **Icons**: Lucide React

## Key file
- `LANDING_SPEC.md` — **READ THIS FIRST**. Every design decision is there.

## Design rules
- Dark theme ONLY (bg: #0A0A0F)
- Glass morphism cards (backdrop-blur + subtle border)
- Purple (#6C5CE7) as primary action color
- Green (#00B894) for positive numbers, Red (#E17055) for Uber/negative
- ALL sections animate on scroll (fade-in + slide-up via Framer Motion)
- Font: Inter for text, JetBrains Mono for numbers/addresses
- Mobile first responsive design

## Coding conventions
- All components in `src/components/`
- Sections are individual components in `sections/`
- Use `'use client'` only where needed (wallet, animations)
- Framer Motion: useInView + motion.div for scroll animations
- Numbers use AnimatedCounter (count-up on viewport enter)
- All wallet logic in `hooks/usePresale.ts`
- Constants (addresses, prices) in `lib/constants.ts`
- Never hardcode wallet addresses in components

## Build order
1. `globals.css` + design tokens
2. UI primitives (Button, Card, GlowCard, GradientText)
3. Layout (Navbar, Footer)
4. Sections top to bottom (Hero → FAQ)
5. Wallet providers + presale logic
6. Animations + polish
7. SEO metadata + OG image
