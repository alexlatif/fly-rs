use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceConfig {
    pub autostart: Option<bool>,
    pub autostop: Option<String>,
    pub concurrency: Option<ConcurrencyConfig>,
    pub ports: Option<Vec<MachinePort>>,
    pub internal_port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AutostopEnum {
    Off,
    Stop,
    Suspend,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConcurrencyConfig {
    pub hard_limit: Option<u32>,
    pub soft_limit: Option<u32>,
    pub concurrency_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ConcurrencyTypeEnum {
    Connections,
    Requests,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachinePort {
    pub end_port: Option<u16>,
    pub force_https: Option<bool>,
    pub handlers: Option<Vec<String>>,
    pub http_options: Option<HttpOptions>,
    pub proxy_proto_options: Option<ProxyProtoOptions>,
    pub start_port: Option<u16>,
    pub tls_options: Option<TlsOptions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpOptions {
    pub compress: Option<bool>,
    pub h2_backend: Option<bool>,
    pub headers_read_timeout: Option<u64>,
    pub idle_timeout: Option<u64>,
    pub response: Option<ResponseOptions>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseOptions {
    pub headers: Option<HashMap<String, String>>,
    pub pristine: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyProtoOptions {
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TlsOptions {
    pub alpn: Option<Vec<String>>,
    pub default_self_signed: Option<bool>,
    pub versions: Option<Vec<String>>,
}
