# DRIDE Presale - Token Presale Contract

The presale contract manages the $DRIDE token presale with enterprise-grade security features.

## Presale Parameters

- **Hard Cap**: 1,000,000 SOL ($1M at $1/SOL)
- **Exchange Rate**: 1 SOL = 200 DRIDE tokens
- **Price**: $0.005 per DRIDE token
- **Minimum Purchase**: 0.1 SOL per transaction
- **Maximum Purchase**: 100 SOL per transaction
- **Per Wallet Limit**: 1,000 SOL total
- **Duration**: Configurable (typically 3 months)

## Architecture

The presale is implemented using Anchor framework with these security features:

### Time Locking
- Purchases only allowed between `start_timestamp` and `end_timestamp`
- Configurable by presale authority
- Prevents early or late purchases

### Hard Cap Enforcement
- Rejects purchases exceeding the total hard cap
- Prevents overselling
- Immutable once set during initialization

### Per-Wallet Limits
- Each wallet limited to 1,000 SOL total
- Prevents whale domination
- Tracked via user PDA

### Emergency Pause
- Authority can pause/unpause presale
- Prevents issues during emergencies
- Requires multisig approval

### Reentrancy Protection
- Effects-Checks-Interactions pattern
- No external calls during purchase
- State validation before and after

## Instructions

### `initialize_presale`
One-time initialization of the presale vault and parameters.

### `purchase_tokens`
Purchase $DRIDE tokens with SOL during the presale window.

### `pause_presale` / `unpause_presale`
Emergency controls to pause/resume the presale.

### `withdraw_sol`
Withdraw SOL from vault after presale ends (authority only).

### `update_presale`
Update presale parameters like end timestamp (authority only).

## Security Features

1. **Time-locked presale**: No purchases before/after defined windows
2. **Hard cap enforcement**: Reject transactions exceeding total supply
3. **Whitelist mechanism**: Optional KYC integration
4. **Anti-front-running**: Price only updates on epoch boundaries
5. **Reentrancy protection**: State checks before/after transfers
6. **Emergency pause**: Authority can halt presale (multisig required)
7. **Per-wallet limits**: Prevent whale domination
8. **Input validation**: All parameters validated
9. **Overflow protection**: Checked arithmetic
10. **Authority controls**: All authority actions require proper signing

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

## State

### PresaleState
- Authority public key (multisig)
- Vault address
- Token mint address
- Hard cap amount
- Total raised amount
- Tokens per SOL rate
- Start/end timestamps
- Pause status

### UserState
- User wallet address
- Total SOL purchased
- Total DRIDE tokens received
- Purchase count
- Last purchase timestamp

## Error Handling

All errors are descriptive and cover:
- Invalid parameters
- Time window violations
- Hard cap exceeded
- Wallet limits exceeded
- Unauthorized actions
- Insufficient funds

## License

Proprietary - All rights reserved
