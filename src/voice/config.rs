#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub pitch: f32,
    pub speaking_rate: f32,
    pub volume_gain_db: f32,
    pub vibrato_amount: Option<f32>,
    pub auto_tune: Option<bool>,
    pub reverb_level: Option<f32>,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            speaking_rate: 1.0,
            volume_gain_db: 0.0,
            vibrato_amount: None,
            auto_tune: None,
            reverb_level: None,
        }
    }
} 