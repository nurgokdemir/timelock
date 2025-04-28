# Timelock Claimable Balance Contract

A simple and functional smart contract implementation for the Soroban platform that demonstrates the **timelock** concept and provides a streamlined **Claimable Balance** mechanism.

## Overview

This contract allows users to:
- Deposit a specified amount of tokens.
- Enable other accounts to claim them either **before** or **after** a specific time point.
- Implement a basic timelock mechanism with a clean and efficient design.

## Features

- Deposit tokens with specified time conditions.
- Set multiple claimant addresses (up to 10).
- Two types of time bounds:
  - **Before**: Claimable only before a specified timestamp.
  - **After**: Claimable only after a specified timestamp.
- Secure authorization checks.
- Simple one-time initialization.

## Contract Structure

### Data Structures

- **DataKey**: Defines storage keys used in the contract.
  - `Init`: Marks the contract as initialized.
  - `Balance`: Stores claimable balance information.

- **TimeBoundKind**: Specifies the type of time constraint:
  - `Before`: Claimable before the timestamp.
  - `After`: Claimable after the timestamp.

- **TimeBound**:
  - `kind`: The type of time bound (`Before` or `After`).
  - `timestamp`: The specific timestamp value.

- **ClaimableBalance**:
  - `token`: Address of the token.
  - `amount`: Token amount.
  - `claimants`: List of addresses that can claim the balance.
  - `time_bound`: Time condition for claiming.

## Technical Details

- Built using the **Soroban SDK** for Stellar's smart contract platform.
- Implemented with `#![no_std]` for efficient operation in blockchain environments.
- Utilizes Rust's type system for safety and reliability.

## Contributing

Contributions are welcome!  
Please feel free to submit a **Pull Request**.
