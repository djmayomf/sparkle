use super::emotion_handler::Emotion;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaraokeSong {
    pub title: String,
    pub artist: String,
    pub lyrics: String,
    pub youtube_id: String,
    pub bpm: f32,
    pub target_pitch: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaraokeState {
    pub is_active: bool,
    pub current_song: Option<KaraokeSong>,
    pub queue: VecDeque<KaraokeSong>,
    pub is_subathon: bool,
}

pub struct KaraokeManager {
    state: KaraokeState,
    voice_tx: mpsc::Sender<String>,
    tts_config: TtsKaraokeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsKaraokeConfig {
    pub pitch_range: (f32, f32),
    pub vibrato_amount: f32,
    pub reverb_level: f32,
    pub auto_tune: bool,
    pub pitch_correction: f32,
    pub expression_level: f32,
}

impl KaraokeManager {
    pub fn new(voice_tx: mpsc::Sender<String>) -> Self {
        Self {
            state: KaraokeState {
                is_active: false,
                current_song: None,
                queue: VecDeque::new(),
                is_subathon: false,
            },
            voice_tx,
            tts_config: TtsKaraokeConfig {
                pitch_range: (-3.0, 3.0),
                vibrato_amount: 0.2,
                reverb_level: 0.15,
                auto_tune: true,
                pitch_correction: 0.7,
                expression_level: 0.8,
            },
        }
    }

    pub async fn start_karaoke(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.state.is_subathon {
            return Err("Karaoke is only available during subathons!".into());
        }

        self.state.is_active = true;
        self.voice_tx.send("Karaoke mode activated! Time to shine! ✨🎤".to_string()).await?;
        Ok(())
    }

    pub async fn queue_song(&mut self, song: KaraokeSong) -> Result<(), Box<dyn std::error::Error>> {
        if !self.state.is_active {
            return Err("Karaoke mode is not active!".into());
        }

        self.state.queue.push_back(song.clone());
        self.voice_tx.send(format!(
            "Added {} by {} to the karaoke queue! 🎵",
            song.title, song.artist
        )).await?;
        Ok(())
    }

    pub async fn process_next_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(song) = self.state.queue.pop_front() {
            self.state.current_song = Some(song.clone());
            
            // Configure TTS for singing
            self.apply_singing_voice_config(&song);
            
            // Announce song start
            self.voice_tx.send(format!(
                "Now singing: {} by {} 🎤✨",
                song.title, song.artist
            )).await?;
        }
        Ok(())
    }

    fn apply_singing_voice_config(&self, song: &KaraokeSong) {
        // Implement more nuanced singing voice configuration
        let pitch_variance = 0.2; // Allow slight pitch variations
        let timing_variance = 0.1; // Allow slight timing variations
        
        // Configure voice settings for "good amateur" level singing
        let voice_settings = VoiceSettings {
            base_pitch: song.target_pitch,
            pitch_variance,
            timing_variance,
            vibrato: self.tts_config.vibrato_amount,
            reverb: self.tts_config.reverb_level,
            auto_tune_strength: 0.7, // Subtle auto-tune
            expression: self.tts_config.expression_level,
        };

        // Apply voice settings
        self.apply_voice_settings(voice_settings);
    }

    async fn process_lyrics_with_timing(&self, song: &KaraokeSong) -> Result<Vec<TimedLyric>, Box<dyn std::error::Error>> {
        let mut timed_lyrics = Vec::new();
        let lines = song.lyrics.lines();
        let mut current_time = 0.0;
        
        for line in lines {
            let duration = self.estimate_line_duration(line, song.bpm);
            timed_lyrics.push(TimedLyric {
                text: line.to_string(),
                start_time: current_time,
                duration,
                is_breath_point: false,
            });
            current_time += duration;
        }

        Ok(timed_lyrics)
    }

    fn estimate_line_duration(&self, line: &str, bpm: f32) -> f32 {
        // Rough estimate: 1 syllable = 1 beat
        let syllable_count = line.split_whitespace()
            .map(|word| self.count_syllables(word))
            .sum::<usize>();
        
        let beats = syllable_count as f32;
        let seconds_per_beat = 60.0 / bpm;
        
        beats * seconds_per_beat
    }

    fn count_syllables(&self, word: &str) -> usize {
        // Simple syllable counting heuristic
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        let mut count = 0;
        let mut prev_was_vowel = false;

        for c in word.chars().flat_map(|c| c.to_lowercase()) {
            let is_vowel = vowels.contains(&c);
            if is_vowel && !prev_was_vowel {
                count += 1;
            }
            prev_was_vowel = is_vowel;
        }

        count.max(1)
    }

    fn add_breath_point(&self, time: f32) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for adding breath points
        Ok(())
    }

    fn apply_singing_params(&self, text: String, params: SingingVoiceParams) -> String {
        // Apply SSML tags for singing parameters
        format!(
            "<speak><prosody pitch='{:+}st' rate='{}%' volume='{:+}dB'>{}</prosody></speak>",
            params.base_pitch,
            (1.0 + params.timing_offset) * 100.0,
            params.expression * 6.0, // Convert to dB
            text
        )
    }

    // Add structure for timed lyrics
    #[derive(Debug, Clone)]
    struct TimedLyric {
        text: String,
        start_time: f32,
        duration: f32,
        is_breath_point: bool,
    }

    // Add method to analyze phrasing
    fn analyze_phrase_timing(&self, lyric: &TimedLyric) -> Result<(), Box<dyn std::error::Error>> {
        // Add natural breathing points
        let words_per_breath = 6..8; // Average number of words before needing a breath
        let phrase_length = lyric.text.split_whitespace().count();
        
        // Calculate if this is a good breathing point
        let needs_breath = phrase_length >= words_per_breath.start 
            && (lyric.text.ends_with(',') || lyric.text.ends_with('.'));

        // Adjust timing slightly for more natural delivery
        if needs_breath {
            // Add a small pause for breath
            self.add_breath_point(lyric.start_time + lyric.duration)?;
        }

        Ok(())
    }

    async fn sing_phrase(&self, lyric: &TimedLyric) -> Result<(), Box<dyn std::error::Error>> {
        // Add slight imperfections for more natural singing
        let timing_variance = (fastrand::f32() - 0.5) * 0.1; // ±50ms variance
        let pitch_variance = (fastrand::f32() - 0.5) * 0.2;  // Slight pitch variation
        
        // Apply natural singing characteristics
        let voice_params = SingingVoiceParams {
            base_pitch: self.tts_config.pitch_range.0 + pitch_variance,
            timing_offset: timing_variance,
            expression: self.tts_config.expression_level,
            vibrato: if lyric.duration > 0.5 { 
                self.tts_config.vibrato_amount 
            } else { 
                0.0 // No vibrato on short notes
            },
        };

        // Sing the phrase with the configured parameters
        self.voice_tx.send(format!(
            "{}",
            self.apply_singing_params(lyric.text.clone(), voice_params)
        )).await?;

        Ok(())
    }

    // Add method to handle the actual singing
    pub async fn perform_current_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(song) = &self.state.current_song {
            // Process lyrics with timing
            let timed_lyrics = self.process_lyrics_with_timing(song).await?;
            
            // Configure voice for this song
            self.apply_singing_voice_config(song);

            // Perform the song
            for lyric in timed_lyrics {
                // Show current lyrics to AI
                self.display_current_lyric(&lyric);
                
                // Wait for the right timing
                tokio::time::sleep(tokio::time::Duration::from_secs_f32(lyric.start_time)).await;
                
                // Sing the phrase
                self.sing_phrase(&lyric).await?;

                // Take a breath if marked
                if lyric.is_breath_point {
                    tokio::time::sleep(tokio::time::Duration::from_secs_f32(0.2)).await;
                }
            }
        }
        Ok(())
    }

    fn display_current_lyric(&self, lyric: &TimedLyric) {
        // Internal display of lyrics for AI processing
        // This doesn't show on stream, only for AI's "reading"
        println!("Current lyric: {}", lyric.text);
    }

    pub async fn stop_karaoke(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.state.is_active = false;
        self.state.current_song = None;
        self.state.queue.clear();
        self.voice_tx.send("Karaoke mode deactivated! Thanks for singing with me! 🎤💕".to_string()).await?;
        Ok(())
    }
} 