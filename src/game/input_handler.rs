use std::collections::HashMap;
use tokio::time::{Duration, sleep};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameCommand {
    pub command: String,
    pub cooldown: Duration,
    pub description: String,
}

pub struct InputHandler {
    commands: HashMap<String, GameCommand>,
    last_used: HashMap<String, std::time::Instant>,
    rate_limits: HashMap<String, (u32, Duration)>, // (max_uses, window)
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            commands: Self::init_commands(),
            last_used: HashMap::new(),
            rate_limits: Self::init_rate_limits(),
        }
    }

    fn init_commands() -> HashMap<String, GameCommand> {
        let mut commands = HashMap::new();
        
        commands.insert("jump".to_string(), GameCommand {
            command: "B".to_string(),
            cooldown: Duration::from_millis(500),
            description: "Make Kamen-Sparkle jump! 🦘".to_string(),
        });
        
        commands.insert("attack".to_string(), GameCommand {
            command: "Y".to_string(),
            cooldown: Duration::from_secs(1),
            description: "Execute an attack move! ⚔️".to_string(),
        });
        
        commands.insert("special".to_string(), GameCommand {
            command: "X+A".to_string(),
            cooldown: Duration::from_secs(5),
            description: "Unleash a special move! ✨".to_string(),
        });
        
        commands
    }

    fn init_rate_limits() -> HashMap<String, (u32, Duration)> {
        let mut limits = HashMap::new();
        limits.insert("jump".to_string(), (3, Duration::from_secs(5)));
        limits.insert("attack".to_string(), (2, Duration::from_secs(3)));
        limits.insert("special".to_string(), (1, Duration::from_secs(30)));
        limits
    }

    pub async fn handle_command(&mut self, command: &str, user: &str) -> Result<String, String> {
        let command = command.to_lowercase();
        
        if let Some(game_command) = self.commands.get(&command) {
            if !self.check_rate_limit(&command, user) {
                return Err("Command is on cooldown! Please wait a moment! (｡•́︿•̀｡)".to_string());
            }
            
            self.execute_command(game_command).await?;
            self.update_rate_limit(&command, user);
            
            Ok(format!("Executed {} command! {}", command, game_command.description))
        } else {
            Err("Unknown command! Try !help for available commands! (◕‿◕✿)".to_string())
        }
    }

    async fn execute_command(&self, command: &GameCommand) -> Result<(), String> {
        // This would integrate with RetroArch client
        // For now, just simulate the command execution
        println!("Executing command: {}", command.command);
        sleep(command.cooldown).await;
        Ok(())
    }

    fn check_rate_limit(&self, command: &str, user: &str) -> bool {
        if let Some(last_use) = self.last_used.get(&format!("{}:{}", command, user)) {
            if let Some((max_uses, window)) = self.rate_limits.get(command) {
                let elapsed = last_use.elapsed();
                if elapsed < *window {
                    return false;
                }
            }
        }
        true
    }

    fn update_rate_limit(&mut self, command: &str, user: &str) {
        self.last_used.insert(
            format!("{}:{}", command, user),
            std::time::Instant::now(),
        );
    }
} 