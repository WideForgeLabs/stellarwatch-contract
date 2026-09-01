use soroban_sdk::{Address, BytesN, String, Vec};

#[derive(Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum HealthStatus {
    Healthy = 0,
    Degraded = 1,
    Unhealthy = 2,
    Unknown = 3,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HealthRecord {
    pub timestamp: u64,
    pub checker: Address,
    pub status: HealthStatus,
    pub response_time_ms: Option<u32>,
    pub message: String,
    pub block_height: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContractMetadata {
    pub name: String,
    pub version: String,
    pub deployer: Address,
    pub registered_at: u64,
    pub last_check: Option<u64>,
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum AlertCondition {
    ResponseTime = 0,
    TTLExpiry = 1,
    StatusChange = 2,
    InvocationFailure = 3,
    Custom = 4,
}

#[derive(Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Severity {
    Info = 0,
    Warn = 1,
    Critical = 2,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlertRule {
    pub id: u64,
    pub contract_id: Option<BytesN<32>>,
    pub condition: AlertCondition,
    pub severity: Severity,
    pub threshold_value: u64,
    pub active: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub notification_target: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlertTriggered {
    pub rule_id: u64,
    pub contract_id: BytesN<32>,
    pub severity: Severity,
    pub message: String,
    pub timestamp: u64,
}