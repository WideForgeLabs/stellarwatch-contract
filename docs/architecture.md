# StellarWatch Architecture

## Overview

StellarWatch monitors Soroban contracts on Stellar. Three smart contracts work together.

## Contract Interactions

See the diagram below.

## Health Recording Flow

1. User calls record_health on health-registry
2. Health-registry checks with contract-registry
3. Health-registry stores record with TTL extension
4. Emits HealthRecorded event

## TTL Extension

All contracts extend TTL on every persistent write.

## Application Layer

The companion repo stellarwatch-app reads from these contracts.
