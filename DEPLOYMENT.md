# Testnet Deployment

Live deployment on Stellar testnet. All three contracts are initialized and functional.

## Contract IDs

| Contract | Testnet Contract ID |
|----------|---------------------|
| contract-registry | CBLZ7ITDN5PGZITZQFWNSIBQVAZDQ7ICOUPQWBJSBYK6GCS2LHL52VSF |
| health-registry | CBH2X5TC2EZ3DPKIOPWL5WCT4AHPLX4DBPZH36B2WMZR35DNMET4BCO3 |
| alert-rules | CC2MBG2PVBR3LNVZXJQWECCU3R5P5STDYPNKPCB3KIHVWABHJDDGNBNO |

## Deployer Public Key

GAFHZ5DNMEVFKISURDFHUGE5BJ5PAF6ZS2TBO6NKL2GO42IPDQIWHHYJ

## Verify on Block Explorer

- contract-registry: https://stellar.expert/explorer/testnet/contract/CBLZ7ITDN5PGZITZQFWNSIBQVAZDQ7ICOUPQWBJSBYK6GCS2LHL52VSF
- health-registry: https://stellar.expert/explorer/testnet/contract/CBH2X5TC2EZ3DPKIOPWL5WCT4AHPLX4DBPZH36B2WMZR35DNMET4BCO3
- alert-rules: https://stellar.expert/explorer/testnet/contract/CC2MBG2PVBR3LNVZXJQWECCU3R5P5STDYPNKPCB3KIHVWABHJDDGNBNO

## Interact With the Contracts

Read the owner:

    stellar contract invoke --network testnet --source-account YOUR_IDENTITY --id CBLZ7ITDN5PGZITZQFWNSIBQVAZDQ7ICOUPQWBJSBYK6GCS2LHL52VSF -- get_owner

## Build and Deploy From Scratch

Build with Rust 1.88 and the wasm32v1-none target:

    cargo +1.88 build --target wasm32v1-none --release

Deploy in dependency order:

    ./scripts/deploy.sh YOUR_IDENTITY

## Notes

- Rust 1.88 or later is required. The wasm32v1-none target produces Soroban-compatible WASM.
- Do not use wasm32-unknown-unknown. It produces WASM with reference-types and multivalue features that Soroban's runtime rejects.
- Each contract's initialize takes an owner: Address parameter. The caller must sign the transaction.
