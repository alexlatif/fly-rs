use crate::machines::{
    CommandResponse, EventResponse, MachineRequest, MachineResponse, MachineState, ProcessResponse,
};
use crate::API_BASE_URL;
use reqwest::Client;
use std::error::Error;
use tracing::debug;

pub struct MachineManager {
    client: Client,
    api_token: String,
}

impl MachineManager {
    pub fn new(client: Client, api_token: String) -> Self {
        Self { client, api_token }
    }

    pub async fn create(
        &self,
        app_name: &str,
        request_data: MachineRequest,
    ) -> Result<MachineResponse, Box<dyn Error>> {
        debug!("Creating machine for app: {}", app_name);
        let url = format!("{}/apps/{}/machines", API_BASE_URL, app_name);

        debug!("Request data: {:#?}", request_data);
        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .header("Content-Type", "application/json")
            .json(&request_data)
            .send()
            .await?;

        debug!("Response: {:#?}", response);
        if response.status().is_success() {
            let response_text = response.text().await?;
            debug!("Raw JSON response body: {}", response_text);

            let response_body: MachineResponse = serde_json::from_str(&response_text)?;
            Ok(response_body)
        } else {
            Err(format!("Request failed with status: {}", response.status()).into())
        }
    }

    pub async fn list(&self, app_name: &str) -> Result<Vec<MachineResponse>, Box<dyn Error>> {
        let url = format!("{}/apps/{}/machines", API_BASE_URL, app_name);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::OK {
            let response_text = response.text().await?;
            debug!("Raw JSON response body: {}", response_text);
            let machines: Vec<MachineResponse> = serde_json::from_str(&response_text)?;
            debug!("List of machines: {:?}", machines);
            Ok(machines)
        } else {
            debug!("Failed to list machines: {:?}", response.status());
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to list machines",
            )))
        }
    }

    pub async fn stop(
        &self,
        app_name: &str,
        machine_id: &str,
        instance_id: &str,
    ) -> Result<(), Box<dyn Error>> {
        debug!("Stopping machine {}", machine_id);
        let url = format!(
            "{}/apps/{}/machines/{}/stop",
            API_BASE_URL, app_name, machine_id
        );

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::OK {
            debug!("Stopped machine {}", machine_id);

            self.wait_for_machine_state(
                app_name,
                machine_id,
                MachineState::Stopped,
                None,
                Some(instance_id),
            )
            .await?;

            Ok(())
        } else {
            debug!(
                "Failed to stop machine {}: {:?}",
                machine_id,
                response.status()
            );
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to stop machine",
            )))
        }
    }

    pub async fn start(&self, app_name: &str, machine_id: &str) -> Result<(), Box<dyn Error>> {
        debug!("Starting machine {}", machine_id);
        let url = format!(
            "{}/apps/{}/machines/{}/start",
            API_BASE_URL, app_name, machine_id
        );

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::OK {
            debug!("Started machine {}", machine_id);
            self.wait_for_machine_state(app_name, machine_id, MachineState::Started, None, None)
                .await?;

            Ok(())
        } else {
            debug!(
                "Failed to start machine {}: {:?}",
                machine_id,
                response.status()
            );
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to start machine",
            )))
        }
    }

    pub async fn delete(
        &self,
        app_name: &str,
        machine_id: &str,
        force: bool,
    ) -> Result<(), Box<dyn Error>> {
        debug!("Deleting machine {}", machine_id);
        let mut url = format!("{}/apps/{}/machines/{}", API_BASE_URL, app_name, machine_id);

        if force {
            url.push_str("?force=true");
        }

        let response = self
            .client
            .delete(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::OK {
            debug!("Deleted machine {}", machine_id);
            self.wait_for_machine_state(app_name, machine_id, MachineState::Destroyed, None, None)
                .await?;

            Ok(())
        } else {
            debug!(
                "Failed to delete machine {}: {:?}",
                machine_id,
                response.status()
            );
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to delete machine",
            )))
        }
    }

    pub async fn wait_for_machine_state(
        &self,
        app_name: &str,
        machine_id: &str,
        desired_state: MachineState,
        timeout: Option<u64>,
        instance_id: Option<&str>,
    ) -> Result<MachineResponse, Box<dyn Error>> {
        debug!(
            "Waiting for machine {} to reach state: {}",
            machine_id, desired_state
        );
        let url = format!(
            "{}/apps/{}/machines/{}/wait",
            API_BASE_URL, app_name, machine_id
        );

        let mut query_params = vec![("state", desired_state.to_string())];

        if let Some(timeout_value) = timeout {
            query_params.push(("timeout", timeout_value.to_string()));
        }

        if let Some(instance_id_value) = instance_id {
            query_params.push(("instance_id", instance_id_value.to_string()));
        }

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .query(&query_params)
            .send()
            .await?;

        if response.status().is_success() {
            let wait_for_state_response: MachineResponse = response.json().await?;
            Ok(wait_for_state_response)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to wait for state: {:?}", response.status()),
            )))
        }
    }

    pub async fn update_machine(
        &self,
        app_name: &str,
        machine_id: &str,
        #[allow(unused_variables)] instance_id: &str,
        machine_request: MachineRequest,
    ) -> Result<MachineResponse, Box<dyn Error>> {
        debug!("Updating machine {}", machine_id);
        let url = format!("{}/apps/{}/machines/{}", API_BASE_URL, app_name, machine_id);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .json(&machine_request)
            .send()
            .await?;

        if response.status().is_success() {
            let machine_response: MachineResponse = response.json().await?;

            // self.wait_for_machine_state(
            //     app_name,
            //     machine_id,
            //     MachineState::Started,
            //     None,
            //     Some(instance_id),
            // )
            // .await?;

            Ok(machine_response)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to update machine: {:?}", response.status()),
            )))
        }
    }

    pub async fn restart_machine(
        &self,
        app_name: &str,
        machine_id: &str,
        instance_id: &str,
    ) -> Result<MachineResponse, Box<dyn Error>> {
        debug!("Restarting machine {}", machine_id);
        let url = format!(
            "{}/apps/{}/machines/{}/restart",
            API_BASE_URL, app_name, machine_id
        );

        let response = self
            .client
            .post(&url) // POST for restarting a machine
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            let machine_response: MachineResponse = response.json().await?;

            self.wait_for_machine_state(
                app_name,
                machine_id,
                MachineState::Started,
                None,
                Some(instance_id),
            )
            .await?;

            Ok(machine_response)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to restart machine: {:?}", response.status()),
            )))
        }
    }

    pub async fn list_events(
        &self,
        app_name: &str,
        machine_id: &str,
    ) -> Result<Vec<EventResponse>, Box<dyn Error>> {
        let url = format!(
            "{}/apps/{}/machines/{}/events",
            API_BASE_URL, app_name, machine_id
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            let events: Vec<EventResponse> = response.json().await?;
            Ok(events)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to list machine events: {:?}", response.status()),
            )))
        }
    }

    pub async fn list_processes(
        &self,
        app_name: &str,
        machine_id: &str,
    ) -> Result<Vec<ProcessResponse>, Box<dyn Error>> {
        let url = format!(
            "{}/apps/{}/machines/{}/ps",
            API_BASE_URL, app_name, machine_id
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            let processes: Vec<ProcessResponse> = response.json().await?;
            Ok(processes)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to list processes: {:?}", response.status()),
            )))
        }
    }

    pub async fn execute_command(
        &self,
        app_name: &str,
        machine_id: &str,
        command: Vec<&str>,
        timeout: Option<u64>,
    ) -> Result<CommandResponse, Box<dyn Error>> {
        debug!(
            "Executing command on machine {} with command: {:?}",
            machine_id, command
        );
        let url = format!(
            "{}/apps/{}/machines/{}/exec",
            API_BASE_URL, app_name, machine_id
        );

        let mut body = serde_json::json!({
            "command": command,
        });
        if let Some(timeout_value) = timeout {
            body["timeout"] = serde_json::json!(timeout_value);
        }

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let command_response: CommandResponse = response.json().await?;
            Ok(command_response)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to execute command: {:?}", response.status()),
            )))
        }
    }
}
