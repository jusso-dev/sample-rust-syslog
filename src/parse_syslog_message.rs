
use std::str::FromStr;
use serde::{Serialize, Deserialize};

// Define a struct to represent a syslog message
#[derive(Debug, Serialize, Deserialize)]
pub struct SyslogMessage {
    severity: u8,
    facility: u8,
    hostname: String,
    app_name: String,
    process_id: String,
    message: String,
}
 impl SyslogMessage {
    pub fn from_str(input: &str) -> Result<SyslogMessage, String> {
        // Split the input string into its individual components
        let mut components = input.splitn(6, ' ');

        // Parse the priority (severity and facility)
        let priority = components.next().ok_or_else(|| "missing priority".to_string())?;
        let priority: u8 = u8::from_str(&priority[1..]).map_err(|_| "invalid priority".to_string())?;
        let severity = priority / 8;
        let facility = priority % 8;

        // Parse the hostname
        let hostname = components.next().ok_or_else(|| "missing hostname".to_string())?;

        // Parse the app name
        let app_name = components.next().ok_or_else(|| "missing app name".to_string())?;

        // Parse the process ID
        let process_id = components.next().ok_or_else(|| "missing process ID".to_string())?;

        // The rest of the input is the message
        let message = components.collect::<Vec<_>>().join(" ");

        Ok(SyslogMessage {
            severity,
            facility,
            hostname: hostname.to_string(),
            app_name: app_name.to_string(),
            process_id: process_id.to_string(),
            message,
        })
    }
}