use google_cloud_tts::{Client, SynthesisInput, VoiceSelectionParams, AudioConfig, AudioEncoding};
use rodio::{OutputStream, Sink};
use serde::{Deserialize, Serialize};

// icu4x -> International Components for Unicode but in Rust
#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub language_code: String,    // "en-US"
    pub name: String,             // "en-US-Studio-O"  (female voice)
    pub pitch: f32,               // -20.0 to 20.0
    pub speaking_rate: f32,       // 0.25 to 4.0
    pub volume_gain_db: f32,      // -96.0 to 16.0
}

pub struct TTSModel {
    client: Client,
    voice_config: VoiceConfig,
    stream_handle: Option<OutputStream>,
    sink: Option<Sink>,
}

impl TTSModel {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new().await?;
        
        // Configure a natural-sounding English female voice
        let voice_config = VoiceConfig {
            language_code: "en-US".to_string(),
            name: "en-US-Studio-O".to_string(), // Female voice with natural tone
            pitch: 0.0,                         // Natural pitch
            speaking_rate: 1.0,                 // Normal speed
            volume_gain_db: 0.0,                // Normal volume
        };

        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(Self {
            client,
            voice_config,
            stream_handle: Some(stream_handle),
            sink: Some(sink),
        })
    }

    pub async fn speak(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let input = SynthesisInput::text(text);
        
        let voice = VoiceSelectionParams {
            language_code: &self.voice_config.language_code,
            name: &self.voice_config.name,
        };

        let audio_config = AudioConfig {
            audio_encoding: AudioEncoding::Linear16,
            pitch: self.voice_config.pitch,
            speaking_rate: self.voice_config.speaking_rate,
            volume_gain_db: self.voice_config.volume_gain_db,
        };

        let response = self.client
            .synthesize_speech(input, voice, audio_config)
            .await?;

        // Play the audio
        if let Some(sink) = &self.sink {
            sink.append(response.audio_content);
            sink.sleep_until_end();
        }

        Ok(())
    }

    pub fn adjust_voice(&mut self, config: VoiceConfig) {
        self.voice_config = config;
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.voice_config.pitch = pitch.max(-20.0).min(20.0);
    }

    pub fn set_speaking_rate(&mut self, rate: f32) {
        self.voice_config.speaking_rate = rate.max(0.25).min(4.0);
    }

    pub fn set_volume(&mut self, volume_db: f32) {
        self.voice_config.volume_gain_db = volume_db.max(-96.0).min(16.0);
    }

    // Method to handle different emotional tones while maintaining English voice
    pub async fn speak_with_emotion(&mut self, text: &str, emotion: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Adjust voice parameters based on emotion while keeping English language
        match emotion {
            "happy" => {
                self.set_pitch(2.0);
                self.set_speaking_rate(1.1);
            },
            "excited" => {
                self.set_pitch(4.0);
                self.set_speaking_rate(1.2);
            },
            "calm" => {
                self.set_pitch(0.0);
                self.set_speaking_rate(0.9);
            },
            "serious" => {
                self.set_pitch(-2.0);
                self.set_speaking_rate(0.95);
            },
            _ => {
                // Reset to default settings
                self.set_pitch(0.0);
                self.set_speaking_rate(1.0);
            }
        }

        self.speak(text).await
    }

    // Method to handle gaming callouts with appropriate urgency
    pub async fn speak_callout(&mut self, text: &str, urgency: u8) -> Result<(), Box<dyn std::error::Error>> {
        // Adjust voice for gaming callouts (faster and clearer)
        let rate = match urgency {
            0..=3 => 1.0,   // Normal pace
            4..=7 => 1.2,   // Slightly urgent
            _ => 1.3,       // Very urgent
        };

        self.set_speaking_rate(rate);
        self.speak(text).await
    }
}
