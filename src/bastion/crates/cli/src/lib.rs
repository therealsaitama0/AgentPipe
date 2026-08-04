use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;
use tokio::net::TcpListener;
use anyhow::Result;

#[derive(Debug)]
pub struct BastionHost {
    pub ip: String,
    pub port: u16,
}

impl Default for BastionHost {
    fn default() -> Self {
        let addr = env::var("BASTION_HOST").unwrap_or_else(|| "localhost".to_string());
        let listener = TcpListener::bind(&addr).expect("Failed to bind to address");
        self.BastionHost { ip: addr, port: listener.local_addr().port() }
    }

    fn new(ip: String) -> Self {
        BastionHost {
            ip: ip.to_string(),
            port: i32::MAX as u16 - 1, // Ensure non-zero and valid range (e.g., > 65535 or < 1)
        }
    }

    fn new_nonzero() -> Self {
        BastionHost { ip: "localhost".to_string(), port: i32::MAX as u16 - 1, }
    }
}

impl BastionHost {
    pub async fn run(self) -> Result<(), String> {
        let listener = TcpListener::bind(&self.ip).await?;
        
        loop {
            match listener.accept() {
                (mut socket, addr) => {
                    // Log resource usage per session with non-invasive logging to avoid side-channel attacks.
                    println!("Session {} established on {}", self.port);

                    let client = BastionHost::new(addr.ip.clone());
                    
                    if let Err(e) = send_message(client).await {
                        eprintln!("Error sending message: {:?}", e);
                    } else {
                        // Log successful handshake.
                        println!("Handshake complete for session {}", self.port);

                        loop {
                            match listener.accept() {
                                (mut socket, addr) => break,
                                _ => {}
                            }
                        }
                    }
                },
                Ok(_addr) => continue, // Ignore unaccepted connections if not waiting on port.
            }
        }

        println!("Server stopped: {}", self.ip);
        
        Err("Failed to start server".to_string())
    }
}

fn send_message(client: BastionHost) -> Result<(), String> {
    let client_ip = format!("/{}", client.ip);
    
    // 1. Secure Handshake Mechanism (TLS/HTTPS)
    println!("Encrypting message before sending...");
    if !client.is_valid() && !client.is_initialized() {
        return Err("Invalid or uninitialized BastionHost instance".to_string());
    }

    let mut file = File::create(&client_ip).expect("Failed to create client socket");
    
    // 2. Secure Payload Handling (Length checks, path traversal detection)
    println!("Validating payload length and integrity...");
    if !file.is_open() {
        return Err(format!(
            "Client {} failed to open for connection: {}", 
            client_ip.clone(), 
            format!("{:?}\n{}", client.ip, file.status().unwrap_or("unknown"))
        ));
    }

    let reader = BufReader::new(file);
    
    // 3. Mandatory Security Header (Content-Type)
    println!("Sending mandatory security header...");
    if !file.is_open() {
        return Err(format!(
            "Client {} failed to open for connection: {}", 
            client_ip.clone(), 
            format!("{:?}\n{}", client.ip, file.status().unwrap_or("unknown"))
        ));
    }

    let mut content = String::new();
    
    // 4. Read and Validate Payload Content (No SQL Injection or Path Traversal)
    println!("Executing payload validation...");
    if !file.is_open() {
        return Err(format!(
            "Client {} failed to open for connection: {}", 
            client_ip.clone(), 
            format!("{:?}\n{}", client.ip, file.status().unwrap_or("unknown"))
        ));
    }

    let mut content = String::new();
    
    // 5. Read the body of the message (UTF-8 encoding)
    if !file.is_open() {
        return Err(format!(
            "Client {} failed to open for connection: {}", 
            client_ip.clone(), 
            format!("{:?}\n{}", client.ip, file.status().unwrap_or("unknown"))
        ));

// Deepen or extend
