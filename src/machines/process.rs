use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessConfig {
    pub cmd: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
    pub env_from: Option<Vec<EnvVarConfig>>,
    pub exec: Option<Vec<String>>,
    pub ignore_app_secrets: Option<bool>,
    pub secrets: Option<Vec<SecretConfig>>,
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvVarConfig {
    pub env_var: String,
    pub field_ref: FieldRefEnum,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FieldRefEnum {
    Id,
    Version,
    AppName,
    PrivateIp,
    Region,
    Image,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretConfig {
    pub env_var: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandResponse {
    pub exit_code: Option<i32>,
    pub exit_signal: Option<i32>,
    pub stderr: Option<String>,
    pub stdout: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessResponse {
    pub command: Option<String>,
    pub cpu: Option<u32>,
    pub directory: Option<String>,
    pub listen_sockets: Option<Vec<ListenSocket>>,
    pub pid: Option<u32>,
    pub rss: Option<u64>,
    pub rtime: Option<u64>,
    pub stime: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListenSocket {
    pub address: Option<String>,
    pub proto: Option<String>,
}
