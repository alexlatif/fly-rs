use crate::{machines::MachineRegions, API_BASE_URL};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tracing::debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Volume {
    pub attached_alloc_id: Option<String>,
    pub attached_machine_id: Option<String>,
    pub auto_backup_enabled: Option<bool>,
    pub block_size: Option<u64>,
    pub blocks: Option<u64>,
    pub blocks_avail: Option<u64>,
    pub blocks_free: Option<u64>,
    pub created_at: Option<String>,
    pub encrypted: Option<bool>,
    pub fstype: Option<String>,
    pub host_status: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub region: Option<String>,
    pub size_gb: Option<u64>,
    pub snapshot_retention: Option<u64>,
    pub state: Option<String>,
    pub zone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Compute {
    pub cpu_kind: Option<String>,
    pub cpus: Option<u32>,
    pub gpu_kind: Option<String>,
    pub gpus: Option<u32>,
    pub host_dedication_id: Option<String>,
    pub kernel_args: Option<Vec<String>>,
    pub memory_mb: Option<u32>,
    pub compute_image: Option<String>,
}
impl Default for Compute {
    fn default() -> Self {
        Self {
            cpu_kind: Some("shared".to_string()),
            cpus: Some(1),
            gpu_kind: None,
            gpus: None,
            host_dedication_id: None,
            kernel_args: None,
            memory_mb: Some(512),
            compute_image: None,
        }
    }
}

impl Compute {
    pub fn new(
        cpu_kind: Option<String>,
        cpus: Option<u32>,
        gpu_kind: Option<String>,
        gpus: Option<u32>,
        host_dedication_id: Option<String>,
        kernel_args: Option<Vec<String>>,
        memory_mb: Option<u32>,
        compute_image: Option<String>,
    ) -> Self {
        Self {
            cpu_kind,
            cpus,
            gpu_kind,
            gpus,
            host_dedication_id,
            kernel_args,
            memory_mb,
            compute_image,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateVolumeRequest {
    pub name: String,
    pub region: MachineRegions,
    pub size_gb: u64,
    pub encrypted: bool,
    pub fstype: String,
    pub require_unique_zone: bool,
    pub compute: Option<Compute>,
    pub snapshot_id: Option<String>,
    pub snapshot_retention: Option<u32>,
    pub source_volume_id: Option<String>,
}

impl CreateVolumeRequest {
    pub fn builder(name: &str, region: MachineRegions, size_gb: u64) -> CreateVolumeRequestBuilder {
        CreateVolumeRequestBuilder::new(name.to_string(), region, size_gb)
    }
}

pub struct CreateVolumeRequestBuilder {
    name: String,
    region: MachineRegions,
    size_gb: u64,
    encrypted: bool,
    fstype: String,
    require_unique_zone: bool,
    compute: Option<Compute>,
    snapshot_id: Option<String>,
    snapshot_retention: Option<u32>,
    source_volume_id: Option<String>,
}

impl CreateVolumeRequestBuilder {
    pub fn new(name: String, region: MachineRegions, size_gb: u64) -> Self {
        Self {
            name,
            region,
            size_gb,
            encrypted: false,
            fstype: "ext4".to_string(),
            require_unique_zone: true,
            compute: Some(Compute::default()),
            snapshot_id: None,
            snapshot_retention: None,
            source_volume_id: None,
        }
    }

    pub fn encrypted(mut self, encrypted: bool) -> Self {
        self.encrypted = encrypted;
        self
    }

    pub fn fstype(mut self, fstype: String) -> Self {
        self.fstype = fstype;
        self
    }

    pub fn require_unique_zone(mut self, require_unique_zone: bool) -> Self {
        self.require_unique_zone = require_unique_zone;
        self
    }

    pub fn compute(mut self, compute: Compute) -> Self {
        self.compute = Some(compute);
        self
    }

    pub fn snapshot_id(mut self, snapshot_id: Option<String>) -> Self {
        self.snapshot_id = snapshot_id;
        self
    }

    pub fn snapshot_retention(mut self, snapshot_retention: Option<u32>) -> Self {
        self.snapshot_retention = snapshot_retention;
        self
    }

    pub fn source_volume_id(mut self, source_volume_id: Option<String>) -> Self {
        self.source_volume_id = source_volume_id;
        self
    }

    pub fn build(self) -> CreateVolumeRequest {
        CreateVolumeRequest {
            name: self.name,
            region: self.region,
            size_gb: self.size_gb,
            encrypted: self.encrypted,
            fstype: self.fstype,
            require_unique_zone: self.require_unique_zone,
            compute: self.compute,
            snapshot_id: self.snapshot_id,
            snapshot_retention: self.snapshot_retention,
            source_volume_id: self.source_volume_id,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateVolumeRequest {
    pub auto_backup_enabled: bool,
    pub snapshot_retention: u64,
}

#[derive(Debug, Serialize)]
pub struct ExtendVolumeRequest {
    pub size_gb: u64,
}

#[derive(Debug, Deserialize)]
pub struct Snapshot {
    pub created_at: String,
    pub digest: String,
    pub id: String,
    pub retention_days: u64,
    pub size: u64,
    pub status: String,
}

pub struct VolumeManager {
    client: Client,
    api_token: String,
}

impl VolumeManager {
    pub fn new(client: Client, api_token: String) -> Self {
        Self { client, api_token }
    }

    pub async fn list_volumes(
        &self,
        app_name: &str,
        summary: bool,
    ) -> Result<Vec<Volume>, Box<dyn Error>> {
        let url = format!(
            "{API_BASE_URL}/apps/{}/volumes?summary={}",
            app_name, summary
        );
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            let volumes = response.json::<Vec<Volume>>().await?;
            debug!("Successfully fetched volumes: {:?}", volumes);
            Ok(volumes)
        } else {
            Err(format!("Failed to fetch volumes: {}", response.status()).into())
        }
    }

    pub async fn create_volume(
        &self,
        app_name: &str,
        volume_request: CreateVolumeRequest,
    ) -> Result<Volume, Box<dyn Error>> {
        debug!("Creating volume: {:?}", volume_request);
        let url = format!("{API_BASE_URL}/apps/{}/volumes", app_name);

        // let payload = serde_json::to_string(&volume_request)?;

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .json(&volume_request)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let volume = response.json::<Volume>().await?;
            Ok(volume)
        } else {
            let error_text = response.text().await?;
            Err(format!("Failed to create volume: {} - {}", status, error_text).into())
        }
    }

    pub async fn get_volume(
        &self,
        app_name: &str,
        volume_id: &str,
    ) -> Result<Volume, Box<dyn Error>> {
        let url = format!("{API_BASE_URL}/apps/{}/volumes/{}", app_name, volume_id);
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            let volume = response.json::<Volume>().await?;
            debug!("Successfully fetched volume details: {:?}", volume);
            Ok(volume)
        } else {
            Err(format!("Failed to fetch volume: {}", response.status()).into())
        }
    }

    pub async fn update_volume(
        &self,
        app_name: &str,
        volume_id: &str,
        update_request: UpdateVolumeRequest,
    ) -> Result<Volume, Box<dyn Error>> {
        let url = format!("{API_BASE_URL}/apps/{}/volumes/{}", app_name, volume_id);
        let response = self
            .client
            .put(&url)
            .bearer_auth(&self.api_token)
            .json(&update_request)
            .send()
            .await?;

        if response.status().is_success() {
            let volume = response.json::<Volume>().await?;
            debug!("Successfully updated volume: {:?}", volume);
            Ok(volume)
        } else {
            Err(format!("Failed to update volume: {}", response.status()).into())
        }
    }

    pub async fn destroy_volume(
        &self,
        app_name: &str,
        volume_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("{API_BASE_URL}/apps/{}/volumes/{}", app_name, volume_id);
        let response = self
            .client
            .delete(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            debug!("Successfully deleted volume with ID: {}", volume_id);
            Ok(())
        } else {
            Err(format!("Failed to delete volume: {}", response.status()).into())
        }
    }

    pub async fn extend_volume(
        &self,
        app_name: &str,
        volume_id: &str,
        extend_request: ExtendVolumeRequest,
    ) -> Result<Volume, Box<dyn Error>> {
        let url = format!(
            "{API_BASE_URL}/apps/{}/volumes/{}/extend",
            app_name, volume_id
        );
        let response = self
            .client
            .put(&url)
            .bearer_auth(&self.api_token)
            .json(&extend_request)
            .send()
            .await?;

        if response.status().is_success() {
            let volume = response.json::<Volume>().await?;
            debug!("Successfully extended volume size: {:?}", volume);
            Ok(volume)
        } else {
            Err(format!("Failed to extend volume size: {}", response.status()).into())
        }
    }

    pub async fn list_snapshots(
        &self,
        app_name: &str,
        volume_id: &str,
    ) -> Result<Vec<Snapshot>, Box<dyn Error>> {
        let url = format!(
            "{API_BASE_URL}/apps/{}/volumes/{}/snapshots",
            app_name, volume_id
        );
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            let snapshots = response.json::<Vec<Snapshot>>().await?;
            debug!("Successfully fetched snapshots: {:?}", snapshots);
            Ok(snapshots)
        } else {
            Err(format!("Failed to fetch snapshots: {}", response.status()).into())
        }
    }

    pub async fn create_snapshot(
        &self,
        app_name: &str,
        volume_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "{API_BASE_URL}/apps/{}/volumes/{}/snapshots",
            app_name, volume_id
        );
        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            debug!("Successfully created snapshot for volume: {}", volume_id);
            Ok(())
        } else {
            Err(format!("Failed to create snapshot: {}", response.status()).into())
        }
    }
}
