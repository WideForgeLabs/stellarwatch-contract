# Contributing to StellarWatch Contracts

Thank you for your interest in contributing. This document outlines the process.

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Create a feature branch
4. Make your changes
5. Run tests
6. Submit a pull request

## Development Setup

Install Rust and the WASM target.

Install Stellar CLI.

Build with cargo build.

Test with cargo test.

## Commit Guidelines

Use conventional commit format.

Types: feat, fix, docs, test, refactor, chore

Examples:
- feat(registry): add contract registration function
- fix(health): resolve TTL extension bug
- test(alert): add unit tests for rule creation

## Pull Request Process

1. Update the README.md if needed
2. Ensure all tests pass
3. Request review from a maintainer
4. Address review feedback

## Code Standards

- No unwrap() outside tests
- No floats - use basis points
- All public functions must have doc comments
- Every function must have a test

## Questions

Open an issue or reach out to the maintainers.