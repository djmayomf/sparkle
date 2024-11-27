use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub emotion: Emotion,
    pub voice_config: VoiceConfig,
    pub timestamp: DateTime<Utc>,
    pub is_speaking: bool,
}

pub struct SyncManager {
    emotion_handler: Arc<Mutex<EmotionHandler>>,
    speech_recognizer: Arc<Mutex<SpeechRecognizer>>,
    tts_model: Arc<Mutex<TTSModel>>,
    current_state: Arc<Mutex<SyncState>>,
    state_tx: mpsc::Sender<SyncState>,
    sync_interval: tokio::time::Duration,
}

impl SyncManager {
    pub fn new(
        emotion_handler: EmotionHandler,
        speech_recognizer: SpeechRecognizer,
        tts_model: TTSModel,
    ) -> (Self, mpsc::Receiver<SyncState>) {
        let (state_tx, state_rx) = mpsc::channel(100);
        
        let manager = Self {
            emotion_handler: Arc::new(Mutex::new(emotion_handler)),
            speech_recognizer: Arc::new(Mutex::new(speech_recognizer)),
            tts_model: Arc::new(Mutex::new(tts_model)),
            current_state: Arc::new(Mutex::new(SyncState {
                emotion: Emotion::Happy,
                voice_config: VoiceConfig::default(),
                timestamp: Utc::now(),
                is_speaking: false,
            })),
            state_tx,
            sync_interval: tokio::time::Duration::from_millis(16), // 60fps sync rate
        };

        // Start sync monitoring
        manager.start_sync_monitor();

        (manager, state_rx)
    }

    fn start_sync_monitor(&self) {
        let emotion_handler = self.emotion_handler.clone();
        let speech_recognizer = self.speech_recognizer.clone();
        let tts_model = self.tts_model.clone();
        let current_state = self.current_state.clone();
        let state_tx = self.state_tx.clone();
        let interval = self.sync_interval;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval);
            
            loop {
                interval.tick().await;
                
                // Lock all components simultaneously to prevent desync
                let mut emotion = emotion_handler.lock().await;
                let mut speech = speech_recognizer.lock().await;
                let mut tts = tts_model.lock().await;
                let mut state = current_state.lock().await;

                // Check for any desync
                if let Some(correction) = Self::check_sync_state(&emotion, &speech, &tts, &state) {
                    // Apply corrections immediately
                    Self::apply_sync_correction(&mut emotion, &mut speech, &mut tts, correction).await;
                    
                    // Update state
                    *state = SyncState {
                        emotion: emotion.get_current_emotion().clone(),
                        voice_config: tts.get_current_config().clone(),
                        timestamp: Utc::now(),
                        is_speaking: tts.is_speaking(),
                    };

                    // Broadcast state update
                    let _ = state_tx.send(state.clone()).await;
                }
            }
        });
    }

    async fn apply_sync_correction(
        emotion: &mut EmotionHandler,
        speech: &mut SpeechRecognizer,
        tts: &mut TTSModel,
        correction: SyncCorrection,
    ) {
        match correction {
            SyncCorrection::EmotionVoiceMismatch(target_emotion) => {
                // Adjust voice to match emotion
                let voice_config = Self::get_voice_config_for_emotion(&target_emotion);
                tts.adjust_voice(voice_config);
                emotion.set_current_emotion(target_emotion);
            },
            SyncCorrection::SpeechDelay => {
                // Reset speech buffer and restart processing
                speech.reset_buffer().await;
                tts.clear_queue().await;
            },
            SyncCorrection::StateMismatch => {
                // Force sync all states
                let current_emotion = emotion.get_current_emotion().clone();
                let voice_config = Self::get_voice_config_for_emotion(&current_emotion);
                tts.adjust_voice(voice_config);
                speech.set_active_state(current_emotion.is_speaking());
            },
        }
    }

    fn get_voice_config_for_emotion(emotion: &Emotion) -> VoiceConfig {
        match emotion {
            Emotion::Happy => VoiceConfig {
                pitch: 2.0,
                speaking_rate: 1.1,
                volume_gain_db: 0.0,
                ..Default::default()
            },
            Emotion::Excited => VoiceConfig {
                pitch: 4.0,
                speaking_rate: 1.2,
                volume_gain_db: 2.0,
                ..Default::default()
            },
            Emotion::Focused => VoiceConfig {
                pitch: 0.0,
                speaking_rate: 0.95,
                volume_gain_db: -1.0,
                ..Default::default()
            },
            // Add configurations for other emotions...
            _ => VoiceConfig::default(),
        }
    }

    pub async fn transition_emotion(&mut self, new_emotion: Emotion) -> Result<(), Box<dyn std::error::Error>> {
        let mut state = self.current_state.lock().await;
        let mut emotion = self.emotion_handler.lock().await;
        let mut tts = self.tts_model.lock().await;

        // Prepare voice change before emotion transition
        let new_voice_config = Self::get_voice_config_for_emotion(&new_emotion);
        tts.prepare_voice_change(new_voice_config.clone()).await?;

        // Execute transition
        emotion.transition_to(new_emotion.clone()).await?;
        tts.apply_prepared_voice_change().await?;

        // Update state
        *state = SyncState {
            emotion: new_emotion,
            voice_config: new_voice_config,
            timestamp: Utc::now(),
            is_speaking: tts.is_speaking(),
        };

        // Broadcast update
        self.state_tx.send(state.clone()).await?;

        Ok(())
    }

    pub async fn force_sync(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut state = self.current_state.lock().await;
        let mut emotion = self.emotion_handler.lock().await;
        let mut speech = self.speech_recognizer.lock().await;
        let mut tts = self.tts_model.lock().await;

        // Reset all components to a known good state
        emotion.reset_to_default().await?;
        speech.reset_buffer().await?;
        tts.clear_queue().await?;

        // Set default state
        *state = SyncState {
            emotion: Emotion::Happy,
            voice_config: VoiceConfig::default(),
            timestamp: Utc::now(),
            is_speaking: false,
        };

        // Broadcast reset
        self.state_tx.send(state.clone()).await?;

        Ok(())
    }
}

#[derive(Debug)]
enum SyncCorrection {
    EmotionVoiceMismatch(Emotion),
    SpeechDelay,
    StateMismatch,
} 