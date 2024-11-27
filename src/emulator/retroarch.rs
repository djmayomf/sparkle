use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct RetroArchConfig {
    pub host: String,
    pub port: u16,
    pub core: String,
    pub rom_path: String,
}

#[derive(Debug)]
pub struct RetroArchClient {
    config: RetroArchConfig,
    connection: Option<TcpStream>,
    button_states: HashMap<String, bool>,
}

impl RetroArchClient {
    pub fn new(config: RetroArchConfig) -> Self {
        Self {
            config,
            connection: None,
            button_states: HashMap::new(),
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        self.connection = Some(TcpStream::connect(addr).await?);
        self.initialize_buttons();
        Ok(())
    }

    fn initialize_buttons(&mut self) {
        for button in ["A", "B", "X", "Y", "UP", "DOWN", "LEFT", "RIGHT", "START", "SELECT"] {
            self.button_states.insert(button.to_string(), false);
        }
    }

    pub async fn load_game(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(conn) = &mut self.connection {
            let command = format!("LOAD_CORE {}\n", self.config.core);
            conn.write_all(command.as_bytes()).await?;
            
            let command = format!("LOAD_CONTENT {}\n", self.config.rom_path);
            conn.write_all(command.as_bytes()).await?;
        }
        Ok(())
    }

    pub async fn press_button(&mut self, button: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(conn) = &mut self.connection {
            let command = format!("INPUT_STATE_P1_{} 1\n", button.to_uppercase());
            conn.write_all(command.as_bytes()).await?;
            self.button_states.insert(button.to_string(), true);
        }
        Ok(())
    }

    pub async fn release_button(&mut self, button: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(conn) = &mut self.connection {
            let command = format!("INPUT_STATE_P1_{} 0\n", button.to_uppercase());
            conn.write_all(command.as_bytes()).await?;
            self.button_states.insert(button.to_string(), false);
        }
        Ok(())
    }

    pub async fn get_game_state(&mut self) -> Result<GameState, Box<dyn std::error::Error>> {
        if let Some(conn) = &mut self.connection {
            let command = "GET_STATUS\n";
            conn.write_all(command.as_bytes()).await?;
            
            let mut buffer = [0; 1024];
            let n = conn.read(&mut buffer).await?;
            let response = String::from_utf8_lossy(&buffer[..n]);
            
            GameState::from_response(&response)
        } else {
            Err("Not connected to RetroArch".into())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub health: u32,
    pub score: u32,
    pub level: u32,
    pub game_over: bool,
}

impl GameState {
    fn from_response(response: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Parse RetroArch response into game state
        // This would need to be customized based on the specific game
        Ok(GameState {
            health: 100,
            score: 0,
            level: 1,
            game_over: false,
        })
    }
} 