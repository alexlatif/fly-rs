use crate::API_BASE_URL;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppResponse {
    pub id: String,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppsResponse {
    pub total_apps: u64,
    pub apps: Vec<App>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub id: String,
    pub name: String,
    pub machine_count: u64,
    pub network: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Organization {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAppRequest {
    pub app_name: String,
    pub org_slug: String,
}

pub struct AppManager {
    client: Client,
    api_token: String,
}

impl AppManager {
    pub fn new(client: Client, api_token: String) -> Self {
        Self { client, api_token }
    }

    pub async fn create(
        &self,
        app_name: &str,
        org_slug: &str,
    ) -> Result<AppResponse, Box<dyn Error>> {
        let url = format!("{}/apps", API_BASE_URL);
        let request_body = CreateAppRequest {
            app_name: app_name.to_string(),
            org_slug: org_slug.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_token)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::CREATED {
            let response_text = response.text().await?;
            let app_response: AppResponse = serde_json::from_str(&response_text)?;
            println!("Created app: {:?}", app_response);
            Ok(app_response)
        } else {
            let error_message = response.text().await?;
            println!("Error response: {}", error_message);
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "App creation failed",
            )))
        }
    }

    pub async fn delete(&self, app_name: &str, force: bool) -> Result<(), Box<dyn Error>> {
        let url = if force {
            format!("{}/apps/{}?force=true", API_BASE_URL, app_name)
        } else {
            format!("{}/apps/{}", API_BASE_URL, app_name)
        };

        let response = self
            .client
            .delete(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::ACCEPTED {
            println!("Deleted app {}", app_name);
        } else {
            println!("Failed to delete {}: {:?}", app_name, response.status());
        }

        Ok(())
    }

    pub async fn list(&self, org_slug: &str) -> Result<Vec<App>, Box<dyn Error>> {
        let url = format!("{}/apps?org_slug={}", API_BASE_URL, org_slug);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::OK {
            let response_text = response.text().await?;
            let apps_response: AppsResponse = serde_json::from_str(&response_text)?;
            println!("List of apps: {:?}", apps_response.apps);
            Ok(apps_response.apps)
        } else {
            println!("Failed to list apps: {:?}", response.status());
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to list apps",
            )))
        }
    }
}
