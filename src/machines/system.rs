use crate::machines::TimeoutConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InitConfig {
    pub cmd: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
    pub exec: Option<Vec<String>>,
    pub kernel_args: Option<Vec<String>>,
    pub swap_size_mb: Option<u64>,
    pub tty: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MountConfig {
    pub add_size_gb: Option<u64>,
    pub encrypted: Option<bool>,
    pub extend_threshold_percent: Option<u64>,
    pub name: Option<String>,
    pub path: String,
    pub size_gb: Option<u64>,
    pub size_gb_limit: Option<u64>,
    pub volume: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileConfig {
    pub guest_path: String,
    pub mode: Option<u32>,
    pub raw_value: Option<String>,
    pub secret_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StaticConfig {
    pub guest_path: String,
    pub url_prefix: String,
    pub index_document: Option<String>,
    pub tigris_bucket: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsConfig {
    pub port: u16,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopConfig {
    pub signal: Option<String>,
    pub timeout: Option<TimeoutConfig>,
}
