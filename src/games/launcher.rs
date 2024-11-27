use std::process::Command;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub name: String,
    pub executable_path: PathBuf,
    pub launch_args: Vec<String>,
    pub steam_app_id: Option<u32>,
    pub required_processes: Vec<String>,
    pub window_title: String,
    pub startup_time: Duration,
}

#[derive(Debug)]
pub struct GameLauncher {
    games: HashMap<String, GameConfig>,
    current_game: Option<String>,
    steam_path: PathBuf,
}

impl GameLauncher {
    pub fn new() -> Self {
        Self {
            games: Self::init_game_configs(),
            current_game: None,
            steam_path: Self::find_steam_path(),
        }
    }

    fn init_game_configs() -> HashMap<String, GameConfig> {
        let mut games = HashMap::new();

        // Overwatch 2 Configuration
        games.insert("overwatch2".to_string(), GameConfig {
            name: "Overwatch 2".to_string(),
            executable_path: PathBuf::from("C:/Program Files (x86)/Overwatch/_retail_/Overwatch.exe"),
            launch_args: vec!["--launch-product=ow2".to_string()],
            steam_app_id: None, // Battle.net game
            required_processes: vec!["Battle.net.exe".to_string()],
            window_title: "Overwatch".to_string(),
            startup_time: Duration::from_secs(30),
        });

        // Valorant Configuration
        games.insert("valorant".to_string(), GameConfig {
            name: "VALORANT".to_string(),
            executable_path: PathBuf::from("C:/Riot Games/VALORANT/live/VALORANT.exe"),
            launch_args: vec![],
            steam_app_id: None, // Riot game
            required_processes: vec!["RiotClientServices.exe".to_string()],
            window_title: "VALORANT".to_string(),
            startup_time: Duration::from_secs(20),
        });

        // Apex Legends Configuration
        games.insert("apex".to_string(), GameConfig {
            name: "Apex Legends".to_string(),
            executable_path: PathBuf::from(""),  // Steam handles this
            launch_args: vec![],
            steam_app_id: Some(1172470),
            required_processes: vec!["r5apex.exe".to_string()],
            window_title: "Apex Legends".to_string(),
            startup_time: Duration::from_secs(45),
        });

        // Minecraft Configuration
        games.insert("minecraft".to_string(), GameConfig {
            name: "Minecraft".to_string(),
            executable_path: PathBuf::from("%APPDATA%/.minecraft/minecraft.exe"),
            launch_args: vec![],
            steam_app_id: None,
            required_processes: vec!["javaw.exe".to_string()],
            window_title: "Minecraft".to_string(),
            startup_time: Duration::from_secs(15),
        });

        // Yu-Gi-Oh! Master Duel Configuration
        games.insert("masterduel".to_string(), GameConfig {
            name: "Yu-Gi-Oh! Master Duel".to_string(),
            executable_path: PathBuf::from(""),  // Steam handles this
            launch_args: vec![],
            steam_app_id: Some(1449850),
            required_processes: vec!["masterduel.exe".to_string()],
            window_title: "Yu-Gi-Oh! Master Duel".to_string(),
            startup_time: Duration::from_secs(20),
        });

        games
    }

    fn find_steam_path() -> PathBuf {
        if cfg!(windows) {
            PathBuf::from("C:/Program Files (x86)/Steam/steam.exe")
        } else if cfg!(target_os = "macos") {
            PathBuf::from("/Applications/Steam.app/Contents/MacOS/steam_osx")
        } else {
            PathBuf::from("/usr/bin/steam")
        }
    }

    pub async fn launch_game(&mut self, game_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = self.games.get(game_id) {
            println!("Launching {}", config.name);

            // Launch through Steam if it's a Steam game
            if let Some(app_id) = config.steam_app_id {
                self.launch_steam_game(app_id).await?;
            } else {
                // Launch directly
                Command::new(&config.executable_path)
                    .args(&config.launch_args)
                    .spawn()?;
            }

            // Wait for game to fully launch
            self.wait_for_game_launch(config).await?;
            self.current_game = Some(game_id.to_string());

            println!("{} launched successfully!", config.name);
            Ok(())
        } else {
            Err("Game not found in configuration".into())
        }
    }

    async fn launch_steam_game(&self, app_id: u32) -> Result<(), Box<dyn std::error::Error>> {
        Command::new(&self.steam_path)
            .arg(format!("steam://rungameid/{}", app_id))
            .spawn()?;
        Ok(())
    }

    async fn wait_for_game_launch(&self, config: &GameConfig) -> Result<(), Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        
        while start_time.elapsed() < config.startup_time {
            if self.is_game_running(config) {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        Err("Game failed to launch within expected time".into())
    }

    fn is_game_running(&self, config: &GameConfig) -> bool {
        // Check if required processes are running
        for process in &config.required_processes {
            if !self.is_process_running(process) {
                return false;
            }
        }
        true
    }

    fn is_process_running(&self, process_name: &str) -> bool {
        if cfg!(windows) {
            Command::new("tasklist")
                .output()
                .map(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .contains(process_name)
                })
                .unwrap_or(false)
        } else {
            Command::new("pgrep")
                .arg(process_name)
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
    }

    pub async fn close_current_game(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(game_id) = &self.current_game {
            if let Some(config) = self.games.get(game_id) {
                println!("Closing {}", config.name);

                // Close the game processes
                for process in &config.required_processes {
                    self.kill_process(process)?;
                }

                self.current_game = None;
                println!("{} closed successfully!", config.name);
            }
        }
        Ok(())
    }

    fn kill_process(&self, process_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if cfg!(windows) {
            Command::new("taskkill")
                .args(&["/F", "/IM", process_name])
                .output()?;
        } else {
            Command::new("pkill")
                .arg(process_name)
                .output()?;
        }
        Ok(())
    }

    pub fn get_current_game(&self) -> Option<&str> {
        self.current_game.as_deref()
    }

    pub fn is_game_installed(&self, game_id: &str) -> bool {
        if let Some(config) = self.games.get(game_id) {
            if let Some(app_id) = config.steam_app_id {
                // Check Steam library
                self.is_steam_game_installed(app_id)
            } else {
                // Check direct installation
                config.executable_path.exists()
            }
        } else {
            false
        }
    }

    fn is_steam_game_installed(&self, app_id: u32) -> bool {
        // Check Steam's libraryfolders.vdf for the game
        if cfg!(windows) {
            let steam_apps = PathBuf::from("C:/Program Files (x86)/Steam/steamapps");
            steam_apps.join(format!("appmanifest_{}.acf", app_id)).exists()
        } else {
            false
        }
    }
} 