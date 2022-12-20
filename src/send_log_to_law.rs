use std::{collections::HashMap};
use reqwest::{Client, header};
use dotenv::dotenv;

use crate::parse_syslog_message::SyslogMessage;

 pub async fn send_request(
    syslog_message: SyslogMessage
) -> Result<(), Box<dyn std::error::Error>> {

    // Load the environment variables from the .env file
    dotenv().ok();

    // Read the SharedKey property from the environment
    let shared_key = std::env::var("SharedKey").expect("SharedKey not found in environment");
    
    // Read the WorkspaceId property from the environment
    let workspace_id = std::env::var("WorkspaceId").expect("WorkspaceId not found in environment");

    // Set up the required headers for the API request
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(
            &shared_key,
        )?,
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/json")?,
    );

    // Create the HTTP client
    let client = Client::new();

    // Set up the data to send to Log Analytics
    let mut data = HashMap::new();
    data.insert("Syslog", syslog_message);

    // Build url for request
    let mut url_start: String = "https://".to_owned();
    let mut url_workspace_id: String = workspace_id.to_owned();
    let url_end: String = ".ods.opinsights.azure.com/api/logs?api-version=2021-03-01-preview".to_owned();
    url_start.push_str(&url_workspace_id);
    url_workspace_id.push_str(&url_end);

    // Send the data to Log Analytics
    let response = client
        .post(&url_end)
        .headers(headers)
        .json(&data)
        .send()
        .await?;

    println!("Response status: {}", response.status());

    Ok(())
}
