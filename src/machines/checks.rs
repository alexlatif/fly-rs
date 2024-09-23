use crate::machines::TimeoutConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Checks {
    pub grace_period: Option<TimeoutConfig>,
    pub headers: Option<Vec<Header>>,
    pub interval: Option<TimeoutConfig>,
    pub kind: Option<CheckKind>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub port: Option<u16>,
    pub protocol: Option<Protocol>,
    pub timeout: Option<TimeoutConfig>,
    pub tls_server_name: Option<String>,
    pub tls_skip_verify: Option<bool>,
    #[serde(rename = "type")]
    pub check_type: Option<CheckType>,
}

impl Checks {
    pub fn new() -> Self {
        Checks {
            grace_period: None,
            headers: None,
            interval: None,
            kind: None,
            method: None,
            path: None,
            port: None,
            protocol: None,
            timeout: None,
            tls_server_name: None,
            tls_skip_verify: None,
            check_type: None,
        }
    }

    pub fn builder() -> CheckBuilder {
        CheckBuilder::new()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CheckKind {
    Informational,
    Readiness,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CheckType {
    Tcp,
    Http,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    Https,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub name: String,
    pub values: Vec<String>,
}

pub struct CheckBuilder {
    grace_period: Option<TimeoutConfig>,
    headers: Option<Vec<Header>>,
    interval: Option<TimeoutConfig>,
    kind: Option<CheckKind>,
    method: Option<String>,
    path: Option<String>,
    port: Option<u16>,
    protocol: Option<Protocol>,
    timeout: Option<TimeoutConfig>,
    tls_server_name: Option<String>,
    tls_skip_verify: Option<bool>,
    check_type: Option<CheckType>,
}

impl CheckBuilder {
    pub fn new() -> Self {
        CheckBuilder {
            grace_period: None,
            headers: None,
            interval: None,
            kind: None,
            method: None,
            path: None,
            port: None,
            protocol: None,
            timeout: None,
            tls_server_name: None,
            tls_skip_verify: None,
            check_type: None,
        }
    }

    pub fn grace_period(mut self, seconds: u64) -> Self {
        self.grace_period = Some(TimeoutConfig::new(seconds));
        self
    }

    pub fn add_header(mut self, name: &str, values: Vec<String>) -> Self {
        let header = Header {
            name: name.to_string(),
            values,
        };
        if let Some(headers) = &mut self.headers {
            headers.push(header);
        } else {
            self.headers = Some(vec![header]);
        }
        self
    }

    pub fn interval(mut self, seconds: u64) -> Self {
        self.interval = Some(TimeoutConfig::new(seconds));
        self
    }

    pub fn kind(mut self, kind: CheckKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn method(mut self, method: &str) -> Self {
        self.method = Some(method.to_string());
        self
    }

    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = Some(protocol);
        self
    }

    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = Some(TimeoutConfig::new(seconds));
        self
    }

    pub fn tls_server_name(mut self, tls_server_name: &str) -> Self {
        self.tls_server_name = Some(tls_server_name.to_string());
        self
    }

    pub fn tls_skip_verify(mut self, tls_skip_verify: bool) -> Self {
        self.tls_skip_verify = Some(tls_skip_verify);
        self
    }

    pub fn check_type(mut self, check_type: CheckType) -> Self {
        self.check_type = Some(check_type);
        self
    }

    pub fn build(self) -> Checks {
        Checks {
            grace_period: self.grace_period,
            headers: self.headers,
            interval: self.interval,
            kind: self.kind,
            method: self.method,
            path: self.path,
            port: self.port,
            protocol: self.protocol,
            timeout: self.timeout,
            tls_server_name: self.tls_server_name,
            tls_skip_verify: self.tls_skip_verify,
            check_type: self.check_type,
        }
    }
}
