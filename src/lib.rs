/// # Unofficial Fly SDK
///
/// The Fly SDK provides an interface for interacting with the Fly.io Machines API.
/// It allows you to manage applications, machines, volumes, and secrets on Fly.io.
/// The SDK is designed to simplify the usage of the Fly.io API with convenient methods for common tasks.
///
/// ## Example
///
/// ```rust
/// use fly_sdk::FlyControl;
///
/// let api_token = "your_api_token".to_string();
/// let fly_control = FlyControl::new(api_token);
///
/// // Now you can use fly_control to manage apps, machines, secrets, and volumes
/// ```
/// The `FlyControl` struct is the main entry point for interacting with the Fly.io API.
/// It encapsulates managers for applications, machines, volumes, and secrets, allowing
/// you to manage these entities using a single unified interface.
///
/// # Fields
/// - `apps`: Manages Fly.io applications.
/// - `machines`: Manages Fly.io machines.
/// - `volumes`: Manages Fly.io volumes.
/// - `secrets`: Manages Fly.io secrets.
///
/// // Example usage: Managing apps, machines, volumes, and secrets
/// // fly_control.apps.create_app(...);
/// // fly_control.machines.list_machines(...);
/// // fly_control.volumes.create_volume(...);
/// // fly_control.secrets.set_secret(...);
/// ```

pub mod apps;
pub mod machines;
pub mod secrets;
pub mod volumes;

use reqwest::Client;

const API_BASE_URL: &str = "https://api.machines.dev/v1";

pub struct FlyControl {
    pub apps: apps::AppManager,
    pub machines: machines::MachineManager,
    pub volumes: volumes::VolumeManager,
    pub secrets: secrets::SecretsManager,
}

impl FlyControl {
    pub fn new(api_token: String) -> Self {
        let client = Client::new();
        FlyControl {
            apps: apps::AppManager::new(client.clone(), api_token.clone()),
            machines: machines::MachineManager::new(client.clone(), api_token.clone()),
            volumes: volumes::VolumeManager::new(client.clone(), api_token.clone()),
            secrets: secrets::SecretsManager::new(client.clone(), api_token.clone()),
        }
    }
}

//     // TOOD feature flags

//     // TODO cargo publish
