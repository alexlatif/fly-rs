use crate::API_BASE_URL;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tracing::debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Secret {
    pub label: String,
    pub publickey: Vec<i32>,
    #[serde(rename = "type")]
    pub stype: String,
}

#[derive(Debug, Serialize)]
pub struct SecretValue {
    pub value: Vec<i32>,
}

impl SecretValue {
    pub fn new(value: Vec<i32>) -> Self {
        Self { value }
    }
}

pub struct SecretsManager {
    client: Client,
    api_token: String,
}

impl SecretsManager {
    pub fn new(client: Client, api_token: String) -> Self {
        Self { client, api_token }
    }

    pub async fn list_secrets(&self, app_name: &str) -> Result<Vec<Secret>, Box<dyn Error>> {
        let url = format!("{API_BASE_URL}/apps/{}/secrets", app_name);
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            let secrets = response.json::<Vec<Secret>>().await?;
            debug!("Successfully fetched secrets: {:?}", secrets);
            Ok(secrets)
        } else {
            Err(format!("Failed to fetch secrets: {}", response.status()).into())
        }
    }

    pub async fn create_secret(
        &self,
        app_name: &str,
        secret_label: &str,
        secret_type: &str,
        value_request: SecretValue,
    ) -> Result<Secret, Box<dyn Error>> {
        debug!("Creating secret: {}", secret_label);
        let url = format!(
            "{API_BASE_URL}/apps/{}/secrets/{}/type/{}",
            app_name, secret_label, secret_type
        );
        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .json(&value_request)
            .send()
            .await?;

        let status = response.status();
        if status.is_success() {
            let secret = response.json::<Secret>().await?;
            Ok(secret)
        } else {
            let error_text = response.text().await?;
            Err(format!("Failed to create secret: {} - {}", status, error_text).into())
        }
    }

    pub async fn generate_secret(
        &self,
        app_name: &str,
        secret_label: &str,
        secret_type: &str,
    ) -> Result<(), Box<dyn Error>> {
        debug!("Generating secret: {}", secret_label);
        let url = format!(
            "{API_BASE_URL}/apps/{}/secrets/{}/type/{}/generate",
            app_name, secret_label, secret_type
        );
        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            debug!("Successfully generated secret: {}", secret_label);
            Ok(())
        } else {
            Err(format!("Failed to generate secret: {}", response.status()).into())
        }
    }

    pub async fn destroy_secret(
        &self,
        app_name: &str,
        secret_label: &str,
    ) -> Result<(), Box<dyn Error>> {
        debug!("Deleting secret: {}", secret_label);
        let url = format!("{API_BASE_URL}/apps/{}/secrets/{}", app_name, secret_label);
        let response = self
            .client
            .delete(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status().is_success() {
            debug!("Successfully deleted secret: {}", secret_label);
            Ok(())
        } else {
            Err(format!("Failed to delete secret: {}", response.status()).into())
        }
    }
}
