use rsyslog::Message;
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
    pub fn from_str(input: &str) -> Result<SyslogMessage, rsyslog::Error> {

        let message: Message = rsyslog::Message::parse(input).map_err(|e| e.to_string()).unwrap();

        Ok(SyslogMessage {
            severity: message.severity,
            facility: message.facility,
            hostname: message.hostname.unwrap_or("").to_string(),
            app_name: message.hostname.unwrap_or("").to_string(),
            process_id: message.proc_id.unwrap_or("").to_string(),
            message: message.msg.msg.to_string()
        })
    }
}