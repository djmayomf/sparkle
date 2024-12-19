use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::language::LanguageConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub input: InputConfig,
    // Add other config sections as needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub enabled: bool,
    // Add other input-specific config options
}

impl Config {
    pub fn load() -> Result<Self> {
        // For now, return a default config
        Ok(Self {
            input: InputConfig {
                enabled: true,
            },
        })
    }
} 