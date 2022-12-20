mod parse_syslog_message;
mod send_log_to_law;

use std::error::Error;
use std::net::{TcpListener};
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // Bind to the syslog port (typically 514)
    let listener = TcpListener::bind("0.0.0.0:514")?;
    
    // Accept incoming connections
    for stream in listener.incoming() {
        let mut stream = stream?;

        // Read the syslog message from the stream
        let mut input = String::new();
        stream.read_to_string(&mut input)?;

        // Print the syslog message to the console
        let message = parse_syslog_message::SyslogMessage::from_str(&input).unwrap();
        println!("Received syslog message: {:?}", &message);

        // Create an asynchronous runtime
        let rt = tokio::runtime::Runtime::new().unwrap();

        // Run the asynchronous function within the runtime
        let result = rt.block_on(send_log_to_law::send_request(message));
        println!("Result: {:?}", result);

        // Send an acknowledgement back to the sender
        stream.write_all(b"ACK\n")?;
    }

    Ok(())
}
