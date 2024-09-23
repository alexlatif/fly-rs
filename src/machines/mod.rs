pub mod api_manager;
pub mod checks;
pub mod endpoints;
pub mod machine;
pub mod networking;
pub mod process;
pub mod regions;
pub mod resources;
pub mod services;
pub mod system;

pub use api_manager::MachineManager;
pub use checks::{CheckKind, CheckType, Checks, Header, Protocol};
pub use endpoints::{EventResponse, MachineRequest, MachineResponse};
pub use machine::{MachineConfig, MachineState};
pub use networking::{DnsConfig, DnsForwardRule};
pub use process::{
    CommandResponse, EnvVarConfig, FieldRefEnum, ProcessConfig, ProcessResponse, SecretConfig,
};
pub use regions::MachineRegions;
pub use resources::{CpuKind, GpuKind, GuestConfig, RestartPolicy, RestartPolicyEnum};
pub use services::ServiceConfig;
pub use system::{FileConfig, InitConfig, MetricsConfig, MountConfig, StaticConfig, StopConfig};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct TimeoutConfig {
    pub duration: u64,
}

impl TimeoutConfig {
    pub fn new(duration: u64) -> Self {
        Self { duration }
    }
}
