use super::scene_generator::{SceneGenerator, SceneType};
use tokio::sync::mpsc;

pub struct SceneManager {
    generator: SceneGenerator,
    scene_channel: mpsc::Sender<SceneCommand>,
    alert_channel: mpsc::Sender<AlertCommand>,
}

pub enum SceneCommand {
    Switch(SceneType),
    UpdateOverlay(String, String),
    ToggleElement(String),
}

pub enum AlertCommand {
    Follow(String),
    Subscribe(String),
    Donation(String, f32),
    Custom(String),
}

impl SceneManager {
    pub async fn new() -> Self {
        let (scene_tx, mut scene_rx) = mpsc::channel(32);
        let (alert_tx, mut alert_rx) = mpsc::channel(32);

        Self {
            generator: SceneGenerator::new(),
            scene_channel: scene_tx,
            alert_channel: alert_tx,
        }
    }

    pub async fn switch_to_gaming(&mut self, game_name: String) -> Result<(), String> {
        self.generator.switch_scene(SceneType::Gaming {
            game_name,
            platform: "PC".to_string(),
        }).await
    }

    pub async fn switch_to_just_chatting(&mut self) -> Result<(), String> {
        self.generator.switch_scene(SceneType::JustChatting).await
    }

    pub async fn switch_to_youtube(&mut self, reaction_mode: bool) -> Result<(), String> {
        self.generator.switch_scene(SceneType::YouTube {
            video_type: "reaction".to_string(),
            reaction_mode,
        }).await
    }

    pub async fn handle_alert(&mut self, alert: AlertCommand) -> Result<(), String> {
        match alert {
            AlertCommand::Follow(username) => {
                // Handle follow alert with appropriate overlay
            }
            AlertCommand::Subscribe(username) => {
                // Handle subscription alert
            }
            AlertCommand::Donation(username, amount) => {
                // Handle donation alert
            }
            AlertCommand::Custom(message) => {
                // Handle custom alert
            }
        }
        Ok(())
    }
} 