# StellarWatch Contracts

On-chain health monitoring and alerting infrastructure for the Stellar ecosystem.

## What This Is

StellarWatch is an open-source monitoring platform for Soroban smart contracts. It tracks contract health (TTL expiry, invocation failures, state drift) and records health states on-chain via Soroban contracts, creating a verifiable audit trail.

This repository contains the Soroban smart contracts that power the platform.

## Architecture

Three contracts form the core:

- **contract-registry**: Tracks which contracts are being monitored
- **health-registry**: Records health check results with on-chain timestamping
- **alert-rules**: Stores configurable threshold rules for automated alerts

## Status

The contracts are implemented and compile to WASM. Deployment to testnet/mainnet is pending RPC stability.

## Repo Structure
contracts/
contract-registry/ # Contract metadata and registration
health-registry/ # Health check records
alert-rules/ # Alert threshold rules
shared/ # Shared types and errors

text

## Build

Run the following command to build the contracts:

```bash
cargo build --target wasm32-unknown-unknown
Tech Stack
Rust (soroban-sdk 21.x)

Soroban smart contracts

WASM target

Maintainers
@Ikechukwu-Patrick

@martinifeanyi058-ship-it

Contributing
See CONTRIBUTING.md for guidelines.

License
MIT

text

Save with `Ctrl+O`, then `Enter`, then exit with `Ctrl+X`.

---

## Step 2: Remove the Leftover tx.xdr File

```bash
rm -f tx.xdr