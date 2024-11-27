use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use crate::obs::controller::OBSController;
use crate::voice::chat_manager::VoiceChatManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSession {
    pub start_time: DateTime<Utc>,
    pub scheduled_end_time: DateTime<Utc>,
    pub is_subathon: bool,
    pub current_scene: String,
    pub stream_title: String,
}

#[derive(Debug, Clone)]
pub struct StreamManager {
    obs_controller: OBSController,
    voice_chat: VoiceChatManager,
    current_session: Option<StreamSession>,
    intro_phrases: Vec<String>,
    outro_phrases: Vec<String>,
    event_sender: broadcast::Sender<StreamEvent>,
    start_delay: Duration,  // 4:20 delay for start
    end_delay: Duration,    // 4:20 delay for end
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamEvent {
    Started(StreamSession),
    SceneChanged(String),
    TimeWarning(Duration),
    Ended(DateTime<Utc>),
}

impl StreamManager {
    pub fn new(obs_controller: OBSController, voice_chat: VoiceChatManager) -> (Self, broadcast::Receiver<StreamEvent>) {
        let (tx, rx) = broadcast::channel(100);
        let manager = Self {
            obs_controller,
            voice_chat,
            current_session: None,
            intro_phrases: vec![
                "Konnichiwa, Kawaii Hackers! Ready for another cyber adventure? (◕‿◕✿)".to_string(),
                "System boot successful! Time to hack into some fun! (｡♥‿♥｡)".to_string(),
                "Initializing stream protocols... Kawaii mode activated! (≧◡≦)".to_string(),
            ],
            outro_phrases: vec![
                "Shutting down for now, but remember: stay safe and kawaii! (◕‿◕✿)".to_string(),
                "Time to recharge my cyber batteries! Thanks for hanging out! (｡♥‿♥｡)".to_string(),
                "Mission accomplished! Catch you next stream! (≧◡≦)".to_string(),
            ],
            event_sender: tx,
            start_delay: Duration::minutes(4) + Duration::seconds(20),
            end_delay: Duration::minutes(4) + Duration::seconds(20),
        };
        (manager, rx)
    }

    pub async fn start_stream(&mut self, title: String, is_subathon: bool) -> Result<(), Box<dyn std::error::Error>> {
        let start_time = Utc::now();
        let duration = if is_subathon {
            Duration::hours(24) // Initial subathon duration
        } else {
            Duration::hours(3) // Regular stream duration
        };

        let session = StreamSession {
            start_time,
            scheduled_end_time: start_time + duration,
            is_subathon,
            current_scene: "Starting Soon".to_string(),
            stream_title: title,
        };

        // Set up starting scene with 4:20 countdown
        self.obs_controller.switch_scene("Starting Soon").await?;
        
        // Start the 4:20 countdown
        let total_seconds = self.start_delay.num_seconds();
        for remaining in (0..=total_seconds).rev() {
            let minutes = remaining / 60;
            let seconds = remaining % 60;
            
            // Update countdown display every second
            if remaining % 10 == 0 { // Update every 10 seconds to avoid spam
                self.voice_chat.add_stream_response(
                    format!("Stream starting in {}:{:02}... (◕‿◕✿)", minutes, seconds)
                );
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        // Play intro animation and voice line
        self.obs_controller.switch_scene("Intro Animation").await?;
        let intro = &self.intro_phrases[fastrand::usize(..self.intro_phrases.len())];
        self.voice_chat.speak(intro).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        // Switch to main scene
        self.obs_controller.switch_scene("Main").await?;
        
        self.current_session = Some(session.clone());
        self.event_sender.send(StreamEvent::Started(session))?;

        // Start time monitoring task
        self.monitor_stream_time();

        Ok(())
    }

    pub async fn end_stream(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(session) = &self.current_session {
            // Play outro animation and voice line
            self.obs_controller.switch_scene("Outro Animation").await?;
            let outro = &self.outro_phrases[fastrand::usize(..self.outro_phrases.len())];
            self.voice_chat.speak(outro).await?;
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            // Switch to ending scene with 4:20 countdown
            self.obs_controller.switch_scene("Stream Ending").await?;
            
            // Start the 4:20 countdown
            let total_seconds = self.end_delay.num_seconds();
            for remaining in (0..=total_seconds).rev() {
                let minutes = remaining / 60;
                let seconds = remaining % 60;
                
                // Update countdown display every 10 seconds
                if remaining % 10 == 0 {
                    self.voice_chat.add_stream_response(
                        format!("Stream ending in {}:{:02}... (｡•́︿•̀｡)", minutes, seconds)
                    );
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }

            self.event_sender.send(StreamEvent::Ended(Utc::now()))?;
            self.current_session = None;
        }
        Ok(())
    }

    fn monitor_stream_time(&mut self) {
        let event_sender = self.event_sender.clone();
        let session = self.current_session.clone();

        tokio::spawn(async move {
            if let Some(session) = session {
                if !session.is_subathon {
                    loop {
                        let now = Utc::now();
                        let time_left = session.scheduled_end_time - now;

                        if time_left <= Duration::zero() {
                            let _ = event_sender.send(StreamEvent::TimeWarning(time_left));
                            break;
                        }

                        // Only send internal events, no chat messages
                        if time_left <= Duration::minutes(30) || time_left <= Duration::minutes(10) {
                            let _ = event_sender.send(StreamEvent::TimeWarning(time_left));
                        }

                        tokio::time::sleep(tokio::time::Duration::from_mins(5)).await;
                    }
                }
            }
        });
    }

    pub async fn change_scene(&mut self, scene_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.obs_controller.switch_scene(scene_name).await?;
        if let Some(session) = &mut self.current_session {
            session.current_scene = scene_name.to_string();
            self.event_sender.send(StreamEvent::SceneChanged(scene_name.to_string()))?;
        }
        Ok(())
    }

    pub fn get_current_session(&self) -> Option<&StreamSession> {
        self.current_session.as_ref()
    }

    pub fn add_intro_phrase(&mut self, phrase: String) {
        self.intro_phrases.push(phrase);
    }

    pub fn add_outro_phrase(&mut self, phrase: String) {
        self.outro_phrases.push(phrase);
    }
} 