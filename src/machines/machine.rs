use crate::machines::{
    Checks, CpuKind, DnsConfig, FileConfig, GpuKind, GuestConfig, InitConfig, MetricsConfig,
    MountConfig, ProcessConfig, RestartPolicy, RestartPolicyEnum, ServiceConfig, StaticConfig,
    StopConfig,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MachineState {
    Started,
    Stopped,
    Suspended,
    Destroyed,
}

impl std::fmt::Display for MachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = match self {
            MachineState::Started => "started",
            MachineState::Stopped => "stopped",
            MachineState::Suspended => "suspended",
            MachineState::Destroyed => "destroyed",
        };
        write!(f, "{}", state_str)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MachineConfig {
    pub image: String,
    pub guest: Option<GuestConfig>,
    pub auto_destroy: Option<bool>,
    pub init: Option<InitConfig>,
    pub env: Option<HashMap<String, String>>,
    pub processes: Option<Vec<ProcessConfig>>,
    pub mounts: Option<Vec<MountConfig>>,
    pub restart: Option<RestartPolicy>,
    pub checks: Option<Checks>,
    pub dns: Option<DnsConfig>,
    pub files: Option<Vec<FileConfig>>,
    pub metadata: Option<HashMap<String, String>>,
    pub metrics: Option<MetricsConfig>,
    pub schedule: Option<String>,
    pub services: Option<Vec<ServiceConfig>>,
    pub standbys: Option<Vec<String>>,
    pub statics: Option<Vec<StaticConfig>>,
    pub stop_config: Option<StopConfig>,
}

impl Default for MachineConfig {
    fn default() -> Self {
        Self {
            image: "ubuntu:20.04".to_string(),
            env: None,
            processes: None,
            init: None,
            auto_destroy: Some(false),
            checks: None,
            dns: None,
            files: None,
            guest: None,
            metadata: None,
            metrics: None,
            mounts: None,
            restart: None,
            schedule: None,
            services: None,
            standbys: None,
            statics: None,
            stop_config: None,
        }
    }
}

impl MachineConfig {
    pub fn builder() -> MachineConfigBuilder {
        MachineConfigBuilder::new()
    }

    pub fn new(
        image: String,
        auto_destroy: Option<bool>,
        guest: Option<GuestConfig>,
        restart: Option<RestartPolicy>,
        env: Option<HashMap<String, String>>,
        processes: Option<Vec<ProcessConfig>>,
        mounts: Option<Vec<MountConfig>>,
        checks: Option<Checks>,
        dns: Option<DnsConfig>,
        files: Option<Vec<FileConfig>>,
        init: Option<InitConfig>,
        metrics: Option<MetricsConfig>,
        schedule: Option<String>,
        services: Option<Vec<ServiceConfig>>,
        standbys: Option<Vec<String>>,
        statics: Option<Vec<StaticConfig>>,
        stop_config: Option<StopConfig>,
    ) -> Self {
        Self {
            image,
            auto_destroy,
            guest,
            restart,
            env,
            processes,
            mounts,
            checks,
            dns,
            files,
            init,
            metadata: None,
            metrics,
            schedule,
            services,
            standbys,
            statics,
            stop_config,
        }
    }
}

pub struct MachineConfigBuilder {
    config: MachineConfig,
}

impl MachineConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: MachineConfig {
                image: "ubuntu:22.04".to_string(),
                auto_destroy: Some(false),
                restart: Some(RestartPolicy::default()),
                guest: Some(GuestConfig::default()),
                ..Default::default()
            },
        }
    }

    pub fn image(mut self, image: &str) -> Self {
        self.config.image = image.to_string();
        self
    }

    pub fn auto_destroy(mut self, auto_destroy: bool) -> Self {
        self.config.auto_destroy = Some(auto_destroy);
        self
    }

    pub fn restart_policy(
        mut self,
        policy: RestartPolicyEnum,
        max_retries: Option<u32>,
        gpu_bid_price: Option<f64>,
    ) -> Self {
        self.config.restart = Some(RestartPolicy {
            policy: policy,
            max_retries,
            gpu_bid_price,
        });
        self
    }

    pub fn cpus(mut self, cpus: u64) -> Self {
        if let Some(guest) = &mut self.config.guest {
            guest.cpus = Some(cpus);
        }
        self
    }

    pub fn memory(mut self, memory_mb: u64) -> Self {
        if let Some(guest) = &mut self.config.guest {
            guest.memory_mb = Some(memory_mb);
        }
        self
    }

    pub fn cpu_kind(mut self, cpu_kind: CpuKind) -> Self {
        if let Some(guest) = &mut self.config.guest {
            guest.cpu_kind = Some(cpu_kind);
        }
        self
    }

    pub fn gpus(mut self, gpus: u64) -> Self {
        if let Some(guest) = &mut self.config.guest {
            guest.gpus = Some(gpus);
        }
        self
    }

    pub fn gpu_kind(mut self, gpu_kind: GpuKind) -> Self {
        if let Some(guest) = &mut self.config.guest {
            guest.gpu_kind = Some(gpu_kind);
        }
        self
    }

    pub fn checks(mut self, config: Checks) -> Self {
        self.config.checks = Some(config);
        self
    }

    pub fn dns(mut self, dns_config: DnsConfig) -> Self {
        self.config.dns = Some(dns_config);
        self
    }

    pub fn add_env(mut self, key: &str, value: &str) -> Self {
        if let Some(env) = &mut self.config.env {
            env.insert(key.to_string(), value.to_string());
        } else {
            let mut env = HashMap::new();
            env.insert(key.to_string(), value.to_string());
            self.config.env = Some(env);
        }
        self
    }

    pub fn add_file(mut self, file_config: FileConfig) -> Self {
        if let Some(files) = &mut self.config.files {
            files.push(file_config);
        } else {
            self.config.files = Some(vec![file_config]);
        }
        self
    }

    pub fn init(mut self, init_config: InitConfig) -> Self {
        self.config.init = Some(init_config);
        self
    }

    pub fn metrics(mut self, metrics_config: MetricsConfig) -> Self {
        self.config.metrics = Some(metrics_config);
        self
    }

    pub fn add_mount(mut self, mount_config: MountConfig) -> Self {
        if let Some(mounts) = &mut self.config.mounts {
            mounts.push(mount_config);
        } else {
            self.config.mounts = Some(vec![mount_config]);
        }
        self
    }

    pub fn add_process(mut self, process_config: ProcessConfig) -> Self {
        if let Some(processes) = &mut self.config.processes {
            processes.push(process_config);
        } else {
            self.config.processes = Some(vec![process_config]);
        }
        self
    }

    pub fn schedule(mut self, schedule: &str) -> Self {
        self.config.schedule = Some(schedule.to_string());
        self
    }

    pub fn add_service(mut self, service_config: ServiceConfig) -> Self {
        if let Some(services) = &mut self.config.services {
            services.push(service_config);
        } else {
            self.config.services = Some(vec![service_config]);
        }
        self
    }

    pub fn add_standby(mut self, standby: &str) -> Self {
        if let Some(standbys) = &mut self.config.standbys {
            standbys.push(standby.to_string());
        } else {
            self.config.standbys = Some(vec![standby.to_string()]);
        }
        self
    }

    pub fn add_static(mut self, static_config: StaticConfig) -> Self {
        if let Some(statics) = &mut self.config.statics {
            statics.push(static_config);
        } else {
            self.config.statics = Some(vec![static_config]);
        }
        self
    }

    pub fn stop_config(mut self, stop_config: StopConfig) -> Self {
        self.config.stop_config = Some(stop_config);
        self
    }

    pub fn build(self) -> MachineConfig {
        self.config
    }
}
