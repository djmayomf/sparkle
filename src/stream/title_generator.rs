use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamTitle {
    pub title: String,
    pub used_at: DateTime<Utc>,
    pub theme: String,
}

pub struct TitleGenerator {
    used_titles: HashSet<String>,
    title_templates: Vec<String>,
    themes: Vec<String>,
}

impl TitleGenerator {
    pub fn new() -> Self {
        Self {
            used_titles: HashSet::new(),
            title_templates: Self::init_templates(),
            themes: Self::init_themes(),
        }
    }

    fn init_templates() -> Vec<String> {
        vec![
            "🌟 {theme} with Kamen-Sparkle! | Kawaii Hacker Time 💻",
            "✨ {theme} Stream | Join the Cyber Adventure! 🔒",
            "💫 {theme} Today! | Hacking with Style 🎀",
            "🎮 {theme} & Chill | Cozy Hacker Stream 💕",
            "🌈 {theme} Special | Kawaii Tech Time! ⚡",
        ].into_iter().map(String::from).collect()
    }

    fn init_themes() -> Vec<String> {
        vec![
            "Coding",
            "Gaming",
            "Cybersecurity",
            "Anime Talk",
            "Tokusatsu Time",
            "Tech Tips",
            "Hacking Practice",
        ].into_iter().map(String::from).collect()
    }

    pub fn generate_title(&mut self) -> StreamTitle {
        loop {
            let theme = &self.themes[fastrand::usize(..self.themes.len())];
            let template = &self.title_templates[fastrand::usize(..self.title_templates.len())];
            
            let title = template.replace("{theme}", theme);
            
            if !self.used_titles.contains(&title) {
                self.used_titles.insert(title.clone());
                return StreamTitle {
                    title,
                    used_at: Utc::now(),
                    theme: theme.to_string(),
                };
            }
        }
    }

    pub fn clear_old_titles(&mut self) {
        // Clear titles older than a week to allow reuse
        self.used_titles.clear();
    }
} 