use twitch_irc::message::ServerMessage;
use tokio::sync::mpsc;
use crate::game::input_handler::InputHandler;
use crate::youtube::manager::YouTubeManager;
use crate::obs::controller::OBSController;
use crate::database::models::{User, ChatMessage};
use sqlx::postgres::PgPool;

pub struct ChatHandler {
    channel_name: String,
    message_tx: mpsc::Sender<ServerMessage>,
    game_input: InputHandler,
    youtube_manager: YouTubeManager,
    obs_controller: OBSController,
}

impl ChatHandler {
    pub fn new(
        channel_name: String,
        message_tx: mpsc::Sender<ServerMessage>,
        game_input: InputHandler,
        youtube_manager: YouTubeManager,
        obs_controller: OBSController,
    ) -> Self {
        Self {
            channel_name,
            message_tx,
            game_input,
            youtube_manager,
            obs_controller,
        }
    }

    pub async fn handle_message(&self, username: &str, message: &str, pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
        // Get or create user
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username) 
             VALUES ($1)
             ON CONFLICT (username) 
             DO UPDATE SET last_seen = CURRENT_TIMESTAMP, interaction_count = users.interaction_count + 1
             RETURNING *"
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        // Store message
        let sentiment = self.analyze_sentiment(message).await?;
        
        sqlx::query(
            "INSERT INTO chat_messages (user_id, content, sentiment) 
             VALUES ($1, $2, $3)"
        )
        .bind(user.id)
        .bind(message)
        .bind(sentiment)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn handle_command(&mut self, command: &str, user: &str) -> Result<(), Box<dyn std::error::Error>> {
        let parts: Vec<&str> = command[1..].split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        match parts[0] {
            // Game commands
            "jump" | "attack" | "special" => {
                self.game_input.handle_command(parts[0], user).await?;
            }

            // Scene commands
            "scene" if parts.len() > 1 => {
                self.obs_controller.switch_scene(parts[1]).await?;
            }

            // YouTube commands
            "search" if parts.len() > 1 => {
                let query = parts[1..].join(" ");
                let videos = self.youtube_manager.search_videos(&query).await?;
                // Handle search results (e.g., display to chat)
            }
            "playlist" if parts.len() > 2 => {
                let action = parts[1];
                let name = parts[2];
                match action {
                    "create" => {
                        // Create new playlist logic
                    }
                    "next" => {
                        if let Some(video) = self.youtube_manager.next_video() {
                            // Play next video logic
                        }
                    }
                    "prev" => {
                        if let Some(video) = self.youtube_manager.previous_video() {
                            // Play previous video logic
                        }
                    }
                    _ => {
                        // Handle unexpected action
                        return Err("Unknown playlist action. Please use 'create', 'next', or 'prev'.".to_string());
                    }
                }
            }
            _ => {
                // Handle unknown command
                return Err("Unknown command! Please try again.".to_string());
            }
        }

        Ok(())
    }
} 