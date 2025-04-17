# Atomic Swap Contract

A smart contract built on the Soroban platform that enables trustless atomic swaps of tokens between two parties.

## Overview

This contract facilitates atomic (indivisible) token exchanges between two different parties. The key feature is that parties don't need to know each other, and their signatures can be matched off-chain. The swap either completes entirely or not at all, eliminating counterparty risk.

## Features

- **Trustless Exchange**: No need for parties to know or trust each other
- **Atomic Execution**: The entire transaction either succeeds completely or fails completely
- **Minimum Price Protection**: Both parties can specify minimum acceptable amounts
- **Refund Mechanism**: Automatically refunds excess tokens to the sender
- **Signature Flexibility**: Signatures can be matched off-chain for greater flexibility

## Contract Structure

### Main Function

#### `swap`
```rust
pub fn swap(
    env: Env,
    a: Address,
    b: Address,
    token_a: Address,
    token_b: Address,
    amount_a: i128,
    min_b_for_a: i128,
    amount_b: i128,
    min_a_for_b: i128,
)
```

This function handles the atomic swap between two parties with the following parameters:
- `env`: Soroban environment
- `a` and `b`: Addresses of the two parties involved in the swap
- `token_a` and `token_b`: Addresses of the tokens to be exchanged
- `amount_a`: Amount of token_a that user A is willing to give
- `min_b_for_a`: Minimum amount of token_b that user A expects in return
- `amount_b`: Amount of token_b that user B is willing to give
- `min_a_for_b`: Minimum amount of token_a that user B expects in return

### Helper Function

#### `move_token`
```rust
fn move_token(
    env: &Env,
    token: &Address,
    from: &Address,
    to: &Address,
    max_spend_amount: i128,
    transfer_amount: i128,
)
```

This internal function manages the token transfer process:
1. Transfers tokens from the sender to the contract
2. Transfers the agreed amount to the recipient
3. Refunds any excess tokens back to the sender

## How It Works

1. **Precondition Verification**:
   - Checks if amount_b ≥ min_b_for_a (ensuring user A's minimum price expectation is met)
   - Checks if amount_a ≥ min_a_for_b (ensuring user B's minimum price expectation is met)

2. **Authorization**:
   - Requires authorization from both parties
   - Uses symmetric arguments, allowing signatures to be used interchangeably

3. **Token Exchange**:
   - Transfers token_a from user A to user B
   - Transfers token_b from user B to user A
   - Automatically refunds any excess tokens

## Technical Details

- Built using the Soroban SDK for Stellar's smart contract platform
- Implemented with `#![no_std]` for efficient operation in blockchain environments
- Uses Rust's type system for safety and reliability
- Symmetric design allows for flexible signature matching

## Security Considerations

- The contract ensures atomic execution, preventing partial swaps
- Authorization is required from both parties
- Minimum price protection prevents unfavorable trades
- The contract acts as a trustless intermediary

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
