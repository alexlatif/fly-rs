use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuestConfig {
    pub cpu_kind: Option<CpuKind>,
    pub cpus: Option<u64>,
    pub gpu_kind: Option<GpuKind>,
    pub gpus: Option<u64>,
    pub memory_mb: Option<u64>,
    pub kernel_args: Option<Vec<String>>,
}

impl Default for GuestConfig {
    fn default() -> Self {
        Self {
            cpu_kind: Some(CpuKind::Shared),
            cpus: Some(1),
            gpu_kind: None,
            gpus: None,
            memory_mb: Some(256),
            kernel_args: None,
        }
    }
}

/// Enum representing different kinds of CPU configurations for machines.
///
/// ### For detailed pricing information:
/// - Please refer to [Fly.io's pricing page](https://fly.io/docs/about/pricing/)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CpuKind {
    // /// Shared CPU with 1 vCPU.
    // /// Available RAM sizes: 256MB, 512MB, 1GB, 2GB
    // SharedCpu1x,

    // /// Shared CPU with 2 vCPUs.
    // /// Available RAM sizes: 512MB, 1GB, 2GB, 4GB
    // SharedCpu2x,

    // /// Shared CPU with 4 vCPUs.
    // /// Available RAM sizes: 1GB, 2GB, 4GB, 8GB
    // SharedCpu4x,

    // /// Shared CPU with 8 vCPUs.
    // /// Available RAM sizes: 2GB, 4GB, 8GB, 16GB
    // SharedCpu8x,
    Shared,
    // /// Performance CPU with 1 vCPU.
    // /// Available RAM sizes: 2GB, 4GB, 8GB
    // Performance1x,

    // /// Performance CPU with 2 vCPUs.
    // /// Available RAM sizes: 4GB, 8GB, 16GB
    // Performance2x,

    // /// Performance CPU with 4 vCPUs.
    // /// Available RAM sizes: 8GB, 16GB, 32GB
    // Performance4x,

    // /// Performance CPU with 8 vCPUs.
    // /// Available RAM sizes: 16GB, 32GB, 64GB
    // Performance8x,

    // /// Performance CPU with 16 vCPUs.
    // /// Available RAM sizes: 32GB, 64GB, 128GB
    // Performance16x,
    Performance,
}

/// Enum representing different kinds of GPU configurations for machines.
///
/// ### For detailed pricing information:
/// - Please refer to [Fly.io's pricing page](https://fly.io/docs/about/pricing/)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuKind {
    A10,

    L40S,

    /// A100 40G PCIe
    A10040GPCIe,

    /// A100 80G SXM
    A10080GSXM,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RestartPolicy {
    pub gpu_bid_price: Option<f64>,
    pub max_retries: Option<u32>,
    pub policy: RestartPolicyEnum,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self {
            gpu_bid_price: None,
            max_retries: None,
            policy: RestartPolicyEnum::No,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RestartPolicyEnum {
    No,
    Always,
    OnFailure,
    SpotPrice,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileConfig {
    pub guest_path: String,
    pub mode: Option<u32>,
    pub raw_value: Option<String>,
    pub secret_name: Option<String>,
}
