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

Contracts are built and attested by the stellar-expert/soroban-build-workflow GitHub Action in `.github/workflows/release.yml`. When a version tag is pushed, the workflow:

1. Compiles each contract in a clean environment
2. Creates a GitHub Release with the optimized WASM
3. Generates a Sigstore attestation linking the WASM to the source commit
4. Sends the attestation to Stellar Expert for verification

To verify a downloaded WASM locally:

    gh attestation verify contract-registry_v0.1.0.wasm --repo WideForgeLabs/stellarwatch-contract

## Notes

- Rust 1.88 or later is required. The wasm32v1-none target produces Soroban-compatible WASM.
- Do not use wasm32-unknown-unknown. It produces WASM with reference-types and multivalue features that Soroban's runtime rejects.
- Each contract's initialize takes an owner: Address parameter. The caller must sign the transaction.
