# StellarWatch Contracts API Reference

Complete reference for the three Soroban contracts in this repository.

- [contract-registry](#contract-registry)
- [health-registry](#health-registry)
- [alert-rules](#alert-rules)

---

## contract-registry

Tracks which contracts are being monitored. Stores metadata per contract: name, version, deployer, registration timestamp, and last health check timestamp.

### Storage Types
```rust
```rust
struct RegistryConfig {
    owner: Address,
    paused: bool,
    }
    
    ```rust
struct ContractMetadata {
    name: String,
    version: String,
    deployer: Address,
    registered_at: u64,
    last_check: Option<u64>,
    active: bool,
    }
    
```

### Functions

#### initialize(env: Env, owner: Address) -> Result<(), Error>

Initializes the registry with an owner address.

- **Auth:** owner.require_auth()
- **Errors:** AlreadyInitialized if called more than once

#### register_contract(env: Env, contract_id: BytesN<32>, name: String, version: String, deployer: Address) -> Result<(), Error>

Registers a contract to be monitored.

- **Auth:** Owner
- **Errors:** NotInitialized, InvalidInput if already registered
- **Events:** ContractRegistered

#### unregister_contract(env: Env, contract_id: BytesN<32>) -> Result<(), Error>

Removes a contract from monitoring.

- **Auth:** Owner
- **Errors:** NotInitialized, ContractNotFound
- **Events:** ContractUnregistered

#### is_registered(env: Env, contract_id: BytesN<32>) -> bool

Checks whether a contract is registered. No auth required.

#### get_owner(env: Env) -> Result<Address, Error>

Returns the owner address. Errors: NotInitialized.

#### is_paused(env: Env) -> Result<bool, Error>

Returns whether the registry is paused. Errors: NotInitialized.

---

## health-registry

Records health check results with on-chain timestamps.

### Storage Types
```rust
```rust
struct HealthConfig {
    registry: Address,
    owner: Address,
    max_history: u32,
    }
    
    ```rust
struct HealthRecord {
    timestamp: u64,
    checker: Address,
    status: HealthStatus,
    response_time_ms: Option<u32>,
    message: String,
    block_height: u32,
    }
    
    ```rust
enum HealthStatus {
    Healthy = 0,
    Degraded = 1,
    Unhealthy = 2,
    Unknown = 3,
    }
    
```

### Functions

#### initialize(env: Env, owner: Address, registry: Address) -> Result<(), Error>

Initializes the health registry.

- **Auth:** owner.require_auth()
- **Errors:** AlreadyInitialized

#### record_health(env: Env, contract_id: BytesN<32>, status: HealthStatus, response_time_ms: Option<u32>, message: String) -> Result<(), Error>

Records a health check result for a contract.

- **Auth:** None
- **Errors:** NotInitialized, MaxHistoryExceeded
- **Events:** HealthRecorded, StatusChanged

#### get_health_history(env: Env, contract_id: BytesN<32>, limit: u32) -> Vec<HealthRecord>

Returns up to limit records, newest first. No auth required.

#### get_latest_health(env: Env, contract_id: BytesN<32>) -> Option<HealthRecord>

Returns the most recent record. No auth required.

---

## alert-rules

Stores configurable threshold rules for automated alerts.

### Storage Types
```rust
```rust
struct AlertConfig {
    registry: Address,
    health_registry: Address,
    owner: Address,
    paused: bool,
    next_rule_id: u64,
    }
    
    ```rust
enum AlertCondition {
    ResponseTime = 0,
    TTLExpiry = 1,
    StatusChange = 2,
    InvocationFailure = 3,
    Custom = 4,
    }
    
    ```rust
enum Severity {
    Info = 0,
    Warn = 1,
    Critical = 2,
    }
    
```

### Functions

#### initialize(env: Env, owner: Address, registry: Address, health_registry: Address) -> Result<(), Error>

Initializes the alert rules.

- **Auth:** owner.require_auth()
- **Errors:** AlreadyInitialized

#### create_rule(env: Env, contract_id: BytesN<32>, condition: AlertCondition, severity: Severity, threshold_value: u64, notification_target: String) -> Result<u64, Error>

Creates a new alert rule. Returns the rule ID.

- **Auth:** Owner
- **Errors:** NotInitialized
- **Events:** RuleCreated

#### delete_rule(env: Env, rule_id: u64) -> Result<(), Error>

Deletes a rule by ID.

- **Auth:** Owner
- **Errors:** NotInitialized, RuleNotFound
- **Events:** RuleDeleted

#### pause_alerts(env: Env) -> Result<(), Error>

Pauses all alert processing. Auth: owner. Events: AlertPaused.

#### resume_alerts(env: Env) -> Result<(), Error>

Resumes alert processing. Auth: owner. Events: AlertResumed.

#### get_owner(env: Env) -> Result<Address, Error>

Returns the owner address.

---

## Errors

| Code | Name | Meaning |
|------|------|---------|
| 1 | NotAuthorized | Caller is not authorized |
| 2 | AlreadyInitialized | Already initialized |
| 3 | NotInitialized | Not initialized |
| 4 | ContractNotFound | Contract does not exist |
| 5 | InvalidInput | Input validation failed |
| 6 | MaxHistoryExceeded | History limit reached |
| 7 | RuleNotFound | Rule does not exist |
| 8 | Paused | Contract is paused |
| 9 | RegistryCallFailed | Registry cross-call failed |
| 10 | HealthCallFailed | Health cross-call failed |

---

## Testnet Contract IDs

| Contract | ID |
|----------|-----|
| contract-registry | CDT3J5O5XQBCFQQWY2SMPWG522QVPXZK7RIHLDLSMFWW7ZQWZN76URNC |
| health-registry | CB53FGQVY5YFSMZP4DHETWIRKOTPLKWYA3SK4Q3PFUFTJU25NZK6WTYC |
| alert-rules | CB7WLFECVP5DOAM3362Q55DSLC5KOKIFJDADNHT47LH4SRUT5MEGMC4W |

See DEPLOYMENT.md for interaction examples.
