# StellarWatch Contracts

![StellarWatch Banner](assets/banner.svg)

On-chain health monitoring and alerting infrastructure for the Stellar ecosystem.

[![CI](https://github.com/WideForgeLabs/stellarwatch-contract/actions/workflows/ci.yml/badge.svg)](https://github.com/WideForgeLabs/stellarwatch-contract/actions/workflows/ci.yml)
[![Testnet](https://img.shields.io/badge/testnet-live-green.svg)](DEPLOYMENT.md)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org/)

## Overview

StellarWatch is an open-source monitoring platform for Soroban smart contracts on the Stellar network. Soroban contracts have TTLs that expire, storage that can drift, and invocation patterns that need watching. No open-source tool exists to monitor these on-chain.

StellarWatch fills this gap by recording health states on-chain via Soroban contracts, creating a verifiable audit trail. Operators configure alert rules stored in contracts, and the platform emits events when thresholds are crossed.

This repository contains the Soroban smart contracts that power the platform.

## Architecture

Three contracts form the core.

### contract-registry

Tracks which contracts are being monitored. Stores metadata per contract: name, version, deployer, registration timestamp, and last health check timestamp.

### health-registry

Records health check results with on-chain timestamping. Each record captures status (Healthy, Degraded, Unhealthy, Unknown), response time, message, and block height. Supports querying history per contract.

### alert-rules

Stores configurable threshold rules for automated alerts. Rules can target specific contracts. Supports pause and resume for maintenance windows.

All contracts implement TTL extension on every persistent write to prevent storage expiration on mainnet. This is a critical requirement for Soroban production deployments.

## Live on Testnet

All three contracts are deployed and initialized on Stellar testnet. See [DEPLOYMENT.md](DEPLOYMENT.md) for contract IDs, explorer links, and interaction examples.

## Current Status

| Contract | Logic | TTL | Tests |
|----------|:-----:|:---:|:-----:|
| contract-registry | Yes | Yes | 4 |
| health-registry | Yes | Yes | 5 |
| alert-rules | Yes | Yes | 5 |
| Total | | | 14 |

CI runs on every push: format check, clippy, WASM build, tests.

## Repo Structure

- contracts/contract-registry - Contract metadata and registration
- contracts/health-registry - Health check records
- contracts/alert-rules - Alert threshold rules
- shared/ - Shared types and errors
- scripts/deploy.sh - Deploy all three contracts in order
- .github/workflows/ci.yml - CI pipeline
- .github/workflows/release.yml - Source verification release workflow

## Requirements

- Rust 1.88 or later
- wasm32v1-none target
- Stellar CLI for deployment

## Build

Run `cargo +1.88 build --target wasm32v1-none --release`

WASM artifacts will be written to target/wasm32v1-none/release/.

Note: Do not use the wasm32-unknown-unknown target. It produces WASM with reference-types and multivalue features that Soroban's runtime rejects.

## Test

Run `cargo test --all`

All 14 tests should pass: 4 for contract-registry, 5 for health-registry, 5 for alert-rules.

## Deploy

First, list your Stellar identities with `stellar keys ls`

Then deploy using the identity name:

    ./scripts/deploy.sh deployer-account

The script deploys in dependency order: contract-registry, then health-registry, then alert-rules. It prints all three contract IDs at the end.

## Source Verification

The repository includes a release workflow (`.github/workflows/release.yml`) that builds reproducible WASM artifacts and generates Sigstore attestations on every version tag. This enables automatic source code verification on block explorers like Stellar Expert.

Attestations are published to GitHub's attestation store and can be inspected via:

    gh attestation verify <wasm-file> --repo WideForgeLabs/stellarwatch-contract

## Tech Stack

- Rust edition 2021
- soroban-sdk 21.x
- WASM target wasm32v1-none
- GitHub Actions for CI

## Maintainers

| Name | Role | Contact |
|------|------|---------|
| [@Ikechukwu-Patrick](https://github.com/Ikechukwu-Patrick) | Lead maintainer, contract architect | [Telegram: @IkSunshine](https://t.me/IkSunshine) |
| [@martinifeanyi058-ship-it](https://github.com/martinifeanyi058-ship-it) | Contract engineer, alert-rules | [Telegram: @threalxavier](https://t.me/threalxavier) |

## Community

Join the StellarForge Developers Telegram group for discussions, questions, and updates:

- [StellarForge_Developers on Telegram](https://t.me/StellarForgeDevCodes)

## Contributing

See CONTRIBUTING.md for guidelines.

Open issues are scoped for contributors. See the issues page at https://github.com/WideForgeLabs/stellarwatch-contract/issues

## Security

See SECURITY.md for vulnerability disclosure.

## License

MIT. See LICENSE for details.
