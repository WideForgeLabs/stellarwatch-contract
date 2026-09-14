# StellarWatch Contracts

On-chain health monitoring and alerting infrastructure for the Stellar ecosystem.

[![CI](https://github.com/WideForgeLabs/stellarwatch-contract/actions/workflows/ci.yml/badge.svg)](https://github.com/WideForgeLabs/stellarwatch-contract/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)

## Overview

StellarWatch is an open-source monitoring platform for Soroban smart contracts on the Stellar network. Soroban contracts have TTLs that expire, storage that can drift, and invocation patterns that need watching. Currently, no open-source tool exists to monitor these on-chain.

StellarWatch fills this gap by recording health states on-chain via Soroban contracts, creating a verifiable audit trail. Operators configure alert rules stored in contracts, and the platform emits events when thresholds are crossed.

This repository contains the Soroban smart contracts that power the platform.

## Architecture

Three contracts form the core:

### contract-registry
Tracks which contracts are being monitored. Stores metadata per contract: name, version, deployer, registration timestamp, and last health check timestamp.

### health-registry
Records health check results with on-chain timestamping. Each record captures status (Healthy, Degraded, Unhealthy, Unknown), response time, message, and block height. Supports querying history per contract.

### alert-rules
Stores configurable threshold rules for automated alerts. Rules can target specific contracts. Supports pause/resume for maintenance windows.

All contracts implement **TTL extension on every persistent write** to prevent storage expiration on mainnet — a critical requirement for Soroban production deployments.

## Current Status

| Contract | Logic | TTL | Tests |
|----------|:-----:|:---:|:-----:|
| contract-registry | Yes | Yes | 4 |
| health-registry | Yes | Yes | 5 |
| alert-rules | Yes | Yes | 5 |
| **Total** | | | **14** |

CI runs on every push: format check, clippy, WASM build, tests.

## Repo Structure
stellarwatch-contract/
├── contracts/
│ ├── contract-registry/ # Contract metadata and registration
│ ├── health-registry/ # Health check records
│ └── alert-rules/ # Alert threshold rules
├── shared/ # Shared types and errors
├── scripts/
│ └── deploy.sh # Deploy all three contracts in order
├── .github/workflows/ci.yml # CI pipeline
├── Cargo.toml # Workspace definition
└── rust-toolchain.toml # Pinned Rust toolchain

text

## Requirements

- Rust 1.80 or later
- `wasm32-unknown-unknown` target
- Stellar CLI (for deployment)

## Build

```bash
cargo build --target wasm32-unknown-unknown --release