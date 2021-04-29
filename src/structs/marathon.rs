#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub app: Option<App>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub id: String,
    pub accepted_resource_roles: Vec<String>,
    pub backoff_factor: f64,
    pub backoff_seconds: i64,
    pub constraints: Vec<Vec<String>>,
    pub container: Container,
    pub cpus: f64,
    pub disk: i64,
    pub env: Env,
    pub executor: String,
    pub health_checks: Vec<HealthCheck>,
    pub instances: i64,
    pub labels: Labels2,
    pub max_launch_delay_seconds: i64,
    pub mem: i64,
    pub gpus: i64,
    pub networks: Vec<Network>,
    pub require_ports: bool,
    pub upgrade_strategy: UpgradeStrategy,
    pub version: String,
    pub version_info: VersionInfo,
    pub kill_selection: String,
    pub unreachable_strategy: UnreachableStrategy,
    pub tasks_staged: i64,
    pub tasks_running: i64,
    pub tasks_healthy: i64,
    pub tasks_unhealthy: i64,
    pub deployments: Vec<::serde_json::Value>,
    pub readiness_check_results: Vec<::serde_json::Value>,
    pub tasks: Vec<Task>,
    pub last_task_failure: LastTaskFailure,
    pub task_stats: TaskStats,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    #[serde(rename = "type")]
    pub type_field: String,
    pub docker: Docker,
    pub volumes: Vec<::serde_json::Value>,
    pub port_mappings: Vec<PortMapping>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Docker {
    pub force_pull_image: bool,
    pub image: String,
    pub parameters: Vec<Parameter>,
    pub privileged: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortMapping {
    pub container_port: i64,
    pub labels: Labels,
    pub name: String,
    pub protocol: String,
    pub service_port: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    #[serde(rename = "RS_LOGLEVEL")]
    pub rs_loglevel: String,
    #[serde(rename = "RS_DEDICATED")]
    pub rs_dedicated: String,
    #[serde(rename = "RS_PASSWORD")]
    pub rs_password: String,
    #[serde(rename = "RS_TOKEN")]
    pub rs_token: String,
    #[serde(rename = "NVIDIA_VISIBLE_DEVICES")]
    pub nvidia_visible_devices: String,
    #[serde(rename = "RS_USERNAME")]
    pub rs_username: String,
    #[serde(rename = "RS_DEBUG")]
    pub rs_debug: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheck {
    pub grace_period_seconds: i64,
    pub interval_seconds: i64,
    pub max_consecutive_failures: i64,
    pub path: String,
    pub port: i64,
    pub protocol: String,
    pub ip_protocol: String,
    pub timeout_seconds: i64,
    pub delay_seconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels2 {
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub name: String,
    pub mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeStrategy {
    pub maximum_over_capacity: i64,
    pub minimum_health_capacity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub last_scaling_at: String,
    pub last_config_change_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnreachableStrategy {
    pub inactive_after_seconds: i64,
    pub expunge_after_seconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub app_id: String,
    pub health_check_results: Vec<HealthCheckResult>,
    pub host: String,
    pub id: String,
    pub ip_addresses: Vec<IpAddress>,
    pub ports: Vec<::serde_json::Value>,
    pub service_ports: Vec<::serde_json::Value>,
    pub slave_id: String,
    pub state: String,
    pub staged_at: String,
    pub started_at: String,
    pub version: String,
    pub local_volumes: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResult {
    pub alive: bool,
    pub consecutive_failures: i64,
    pub first_success: String,
    pub instance_id: String,
    pub last_success: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpAddress {
    pub ip_address: String,
    pub protocol: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastTaskFailure {
    pub app_id: String,
    pub host: String,
    pub message: String,
    pub state: String,
    pub task_id: String,
    pub timestamp: String,
    pub version: String,
    pub slave_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStats {
    pub started_after_last_scaling: StartedAfterLastScaling,
    pub with_latest_config: WithLatestConfig,
    pub total_summary: TotalSummary,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartedAfterLastScaling {
    pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub counts: Counts,
    pub life_time: LifeTime,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counts {
    pub staged: i64,
    pub running: i64,
    pub healthy: i64,
    pub unhealthy: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifeTime {
    pub average_seconds: f64,
    pub median_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithLatestConfig {
    pub stats: Stats2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats2 {
    pub counts: Counts2,
    pub life_time: LifeTime2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counts2 {
    pub staged: i64,
    pub running: i64,
    pub healthy: i64,
    pub unhealthy: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifeTime2 {
    pub average_seconds: f64,
    pub median_seconds: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalSummary {
    pub stats: Stats3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats3 {
    pub counts: Counts3,
    pub life_time: LifeTime3,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counts3 {
    pub staged: i64,
    pub running: i64,
    pub healthy: i64,
    pub unhealthy: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifeTime3 {
    pub average_seconds: f64,
    pub median_seconds: f64,
}
