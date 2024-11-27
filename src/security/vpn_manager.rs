use std::env;
use dotenv::dotenv;
use tokio::process::Command;

pub struct VPNManager {
    config_path: String,
    is_connected: bool,
}

impl VPNManager {
    pub fn new() -> Self {
        dotenv().ok(); // Load .env file
        
        Self {
            config_path: env::var("VPN_CONFIG_PATH").unwrap_or_default(),
            is_connected: false,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Get credentials from secure environment variables
        let username = env::var("VPN_USERNAME")?;
        let password = env::var("VPN_PASSWORD")?;

        // Use credentials securely
        Command::new("openvpn")
            .arg("--config")
            .arg(&self.config_path)
            .arg("--auth-user-pass")
            .spawn()?;

        self.is_connected = true;
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Command::new("killall")
            .arg("openvpn")
            .spawn()?;

        self.is_connected = false;
        Ok(())
    }
} 