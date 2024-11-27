use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubathonConfig {
    pub is_affiliate: bool,
    pub base_duration: Duration,
    pub time_per_sub: Duration,
    pub time_per_bits: Duration,
    pub goals: Vec<SubathonGoal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubathonGoal {
    pub subs_required: u32,
    pub reward_description: String,
    pub completed: bool,
}

#[derive(Debug)]
pub struct SubathonManager {
    config: SubathonConfig,
    start_time: Option<SystemTime>,
    end_time: Option<SystemTime>,
    current_subs: u32,
    cached_titles: Vec<String>,
}

impl SubathonManager {
    pub fn new(config: SubathonConfig) -> Self {
        Self {
            config,
            start_time: None,
            end_time: None,
            current_subs: 0,
            cached_titles: Vec::new(),
        }
    }

    pub fn start_subathon(&mut self) -> Result<(), &'static str> {
        if !self.config.is_affiliate {
            return Err("Cannot start subathon: Not a Twitch affiliate");
        }

        let now = SystemTime::now();
        self.start_time = Some(now);
        self.end_time = Some(now + self.config.base_duration);
        Ok(())
    }

    pub fn add_subscription(&mut self) {
        self.current_subs += 1;
        if let Some(end_time) = self.end_time {
            self.end_time = Some(end_time + self.config.time_per_sub);
        }
        self.check_goals();
    }

    fn check_goals(&mut self) {
        for goal in &mut self.config.goals {
            if self.current_subs >= goal.subs_required && !goal.completed {
                goal.completed = true;
                // Trigger celebration animation and announcement
            }
        }
    }

    pub fn generate_stream_title(&mut self) -> String {
        let titles = vec![
            "🌟 Kawaii Hackers Subathon! Join the cyber adventure! 💻",
            "✨ Subathon with your favorite VTuber hacker! 🔒",
            "🎮 Level up the stream with subs! Kawaii Hacker style! 💕",
        ];

        loop {
            let title = titles[fastrand::usize(..titles.len())].to_string();
            if !self.cached_titles.contains(&title) {
                self.cached_titles.push(title.clone());
                return title;
            }
        }
    }
} 