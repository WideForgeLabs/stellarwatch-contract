# Testnet Deployment

Live deployment on Stellar testnet. All three contracts are initialized and functional.

## Contract IDs

| Contract | Testnet Contract ID |
|----------|---------------------|
| contract-registry | CDT3J5O5XQBCFQQWY2SMPWG522QVPXZK7RIHLDLSMFWW7ZQWZN76URNC |
| health-registry | CB53FGQVY5YFSMZP4DHETWIRKOTPLKWYA3SK4Q3PFUFTJU25NZK6WTYC |
| alert-rules | CB7WLFECVP5DOAM3362Q55DSLC5KOKIFJDADNHT47LH4SRUT5MEGMC4W |

## Deployer Public Key

GAFHZ5DNMEVFKISURDFHUGE5BJ5PAF6ZS2TBO6NKL2GO42IPDQIWHHYJ

## Verify on Block Explorer

- contract-registry: https://stellar.expert/explorer/testnet/contract/CDT3J5O5XQBCFQQWY2SMPWG522QVPXZK7RIHLDLSMFWW7ZQWZN76URNC
- health-registry: https://stellar.expert/explorer/testnet/contract/CB53FGQVY5YFSMZP4DHETWIRKOTPLKWYA3SK4Q3PFUFTJU25NZK6WTYC
- alert-rules: https://stellar.expert/explorer/testnet/contract/CB7WLFECVP5DOAM3362Q55DSLC5KOKIFJDADNHT47LH4SRUT5MEGMC4W

## Interact With the Contracts

Read the owner:

    stellar contract invoke --network testnet --source-account YOUR_IDENTITY --id CDT3J5O5XQBCFQQWY2SMPWG522QVPXZK7RIHLDLSMFWW7ZQWZN76URNC -- get_owner

## Build and Deploy From Scratch

Build with Rust 1.88 and the wasm32v1-none target:

    cargo +1.88 build --target wasm32v1-none --release

Deploy in dependency order:

    ./scripts/deploy.sh YOUR_IDENTITY

## Source Verification

Source code verification via Stellar Expert requires contracts to be deployed from a WASM artifact produced by the official `stellar-expert/soroban-build-workflow`. This repo includes `.github/workflows/release.yml` that triggers on version tags.

## Notes

- Rust 1.88 or later is required. The wasm32v1-none target produces Soroban-compatible WASM.
- Do not use wasm32-unknown-unknown. It produces WASM with reference-types and multivalue features that Soroban's runtime rejects.
- Each contract's initialize takes an owner: Address parameter. The caller must sign the transaction.
