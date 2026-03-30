# DRIDE Token - Solana SPL Token

The $DRIDE token is the native utility token of the dRide decentralized ride-sharing protocol.

## Token Specifications

- **Symbol**: DRIDE
- **Name**: DRIDE
- **Decimals**: 9
- **Total Supply**: 1,000,000,000 (1 billion)
- **Standard**: SPL Token (Solana Program Library)
- **Mint Authority**: Multisig PDA (3/5 required)
- **Freeze Authority**: None (tokens are immediately transferable after TGE)

## Architecture

The token is implemented using Anchor framework on Solana:

- **Mint Account**: Program Derived Address (PDA) from `["dride_mint"]` seed
- **Mint Authority**: PDA from `["mint_authority"]` seed (controlled by Squads multisig)
- **Security**: Max supply hardcoded to 1 billion tokens, immutable after deployment

## Instructions

### `initialize_mint`
One-time initialization of the token mint with fixed supply.

### `mint_tokens`
Mint new tokens to a recipient account. Only callable by the mint authority (multisig).

### `transfer_tokens`
Standard SPL token transfer between accounts.

## Deployment

```bash
# Build the program
anchor build

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Run tests
anchor test

# Verify program
solana program show <PROGRAM_ID> --url devnet
```

## Security Features

1. **Fixed Supply**: Hardcoded max supply of 1 billion tokens (immutable)
2. **Multisig Authority**: Minting requires 3/5 multisig approval via Squads
3. **No Freeze Authority**: Tokens are fully transferable after TGE
4. **Input Validation**: All amounts and parameters validated
5. **Overflow Protection**: Checked arithmetic for all operations

## Tokenomics Distribution

- **30% Presale & Public Sale**: 300M tokens at $0.005
- **25% Ecosystem & Rewards**: 250M tokens for driver/passenger incentives
- **20% Team & Advisors**: 200M tokens (1yr cliff, 4yr linear vesting)
- **15% Liquidity Pool**: 150M tokens (Raydium/Orca DEX locked)
- **10% Treasury/Reserve**: 100M tokens (Squads multisig controlled)

## License

Proprietary - All rights reserved
