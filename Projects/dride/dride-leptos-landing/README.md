# dRide Landing Page - Rust + Leptos

Enterprise-grade landing page for the dRide decentralized ride-sharing protocol, built with Rust + Leptos and comprehensive security.

## Architecture

- **Frontend**: Leptos 0.7 (Rust-based reactive framework)
- **Security**: OWASP Top 10 compliance, Zero Trust architecture
- **Blockchain**: Solana integration with multi-wallet support
- **Smart Contracts**: Anchor-based token and presale programs

## Features

- ✅ Real $DRIDE token purchase functionality
- ✅ Multi-wallet support (Phantom, Solflare, Backpack, Glow, Brave, Trust, Coinbase)
- ✅ Comprehensive security (OWASP Top 10)
- ✅ Zero Trust input validation
- ✅ Rate limiting and DoS protection
- ✅ XSS/CSRF/SQL injection prevention
- ✅ Internationalization (en/pt)
- ✅ Responsive design (mobile/tablet/desktop)

## Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk
cargo install trunk

# Install wasm-pack
cargo install wasm-pack
```

### Setup

```bash
# Copy environment variables
cp .env.example .env
# Edit .env with your values

# Build the project
trunk build

# Run development server
trunk serve

# Open browser to http://localhost:3000
```

### Security Features

**OWASP Top 10 Mitigation:**

1. **A01: Broken Access Control** - RBAC, server-side validation
2. **A02: Cryptographic Failures** - ed25519, SHA-256, TLS 1.3
3. **A03: Injection** - Input validation, parameterized queries, XSS sanitization
4. **A04: Insecure Design** - Time-locked presale, hard cap, rate limiting
5. **A05: Security Misconfiguration** - CSP, HSTS, secure headers
6. **A06: Vulnerable Components** - `cargo audit`, pinned dependencies
7. **A07: Authentication Failures** - Wallet signature verification
8. **A08: Data Integrity Failures** - SRI, transaction verification
9. **A09: Logging Failures** - Comprehensive security event logging
10. **A10: SSRF** - URL whitelist, private IP blocking

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

### Security Audit

```bash
# Run security audit
cargo audit

# Run clippy with security lints
cargo clippy --all-targets --all-features -- \
    -W clippy::missing_errors_doc \
    -W clippy::missing_panics_doc \
    -W clippy::unwrap_used \
    -W clippy::expect_used

# Check for outdated dependencies
cargo outdated
```

## Deployment

### Build for Production

```bash
# Optimized build
trunk build --release

# The output is in ./dist
```

### Deploy

```bash
# Deploy to Vercel
vercel deploy dist

# Or use any static hosting service:
# - Vercel
# - Netlify
# - Cloudflare Pages
# - GitHub Pages
```

## Smart Contracts

The landing page integrates with these smart contracts:

- **dRide Token**: SPL Token with fixed 1B supply
- **dRide Presale**: Secure presale with time-locking and hard cap

See:
- `/dride-token/` - Token program
- `/dride-presale/` - Presale program

## Contributing

See [CLAUDE.md](../CLAUDE.md) for project guidelines.

## License

Proprietary - All rights reserved
