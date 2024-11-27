use obws::{Client as OBSClient, requests::scenes::SetCurrentScene};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OBSPlatform {
    OBSStudio,
    StreamlabsOBS,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OBSConfig {
    pub platform: OBSPlatform,
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub scenes: HashMap<String, SceneConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneConfig {
    pub name: String,
    pub elements: Vec<SceneElement>,
    pub transitions: HashMap<String, String>,
    pub platform_specific: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneElement {
    pub name: String,
    pub source_type: String,
    pub position: Position,
    pub size: Size,
    pub settings: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub struct UniversalOBSManager {
    config: OBSConfig,
    obs_client: OBSClient,
    current_scene: String,
    platform_specific_handlers: HashMap<OBSPlatform, Box<dyn OBSPlatformHandler>>,
}

trait OBSPlatformHandler: Send + Sync {
    fn set_scene(&self, scene_name: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn update_source(&self, source_name: &str, settings: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>>;
    fn get_platform_name(&self) -> &'static str;
}

impl UniversalOBSManager {
    pub async fn new(config: OBSConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let obs_client = OBSClient::connect(
            format!("ws://{}:{}", config.host, config.port),
            config.password.clone(),
        ).await?;

        let mut platform_specific_handlers = HashMap::new();
        match config.platform {
            OBSPlatform::OBSStudio => {
                platform_specific_handlers.insert(
                    OBSPlatform::OBSStudio,
                    Box::new(OBSStudioHandler::new(&config)) as Box<dyn OBSPlatformHandler>
                );
            },
            OBSPlatform::StreamlabsOBS => {
                platform_specific_handlers.insert(
                    OBSPlatform::StreamlabsOBS,
                    Box::new(StreamlabsHandler::new(&config)) as Box<dyn OBSPlatformHandler>
                );
            }
        }

        Ok(Self {
            config,
            obs_client,
            current_scene: String::new(),
            platform_specific_handlers,
        })
    }

    pub async fn switch_scene(&mut self, scene_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(scene_config) = self.config.scenes.get(scene_name) {
            // Get platform-specific handler
            let handler = self.platform_specific_handlers.get(&self.config.platform)
                .ok_or("Platform handler not found")?;

            // Handle platform-specific scene switching
            handler.set_scene(scene_name)?;

            // Common OBS WebSocket command
            self.obs_client.send(SetCurrentScene {
                name: scene_name.to_string(),
            }).await?;

            self.current_scene = scene_name.to_string();

            // Apply any platform-specific settings
            if let Some(settings) = scene_config.platform_specific.get(handler.get_platform_name()) {
                println!("Applying platform-specific settings for {}: {}", handler.get_platform_name(), settings);
            }
        }
        Ok(())
    }

    pub async fn update_scene_element(
        &mut self,
        scene_name: &str,
        element_name: &str,
        settings: HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(scene) = self.config.scenes.get(scene_name) {
            if let Some(element) = scene.elements.iter().find(|e| e.name == element_name) {
                let handler = self.platform_specific_handlers.get(&self.config.platform)
                    .ok_or("Platform handler not found")?;

                // Merge default and new settings
                let mut final_settings = element.settings.clone();
                final_settings.extend(settings);

                handler.update_source(element_name, &final_settings)?;
            }
        }
        Ok(())
    }

    pub fn get_current_scene(&self) -> &str {
        &self.current_scene
    }

    pub fn is_streamlabs(&self) -> bool {
        matches!(self.config.platform, OBSPlatform::StreamlabsOBS)
    }
}

struct OBSStudioHandler {
    config: OBSConfig,
}

impl OBSStudioHandler {
    fn new(config: &OBSConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl OBSPlatformHandler for OBSStudioHandler {
    fn set_scene(&self, scene_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // OBS Studio specific scene switching logic
        Ok(())
    }

    fn update_source(&self, source_name: &str, settings: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        // OBS Studio specific source update logic
        Ok(())
    }

    fn get_platform_name(&self) -> &'static str {
        "OBS Studio"
    }
}

struct StreamlabsHandler {
    config: OBSConfig,
}

impl StreamlabsHandler {
    fn new(config: &OBSConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl OBSPlatformHandler for StreamlabsHandler {
    fn set_scene(&self, scene_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Streamlabs specific scene switching logic
        // Handle any Streamlabs-specific scene properties
        Ok(())
    }

    fn update_source(&self, source_name: &str, settings: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        // Streamlabs specific source update logic
        // Handle Streamlabs-specific source properties
        Ok(())
    }

    fn get_platform_name(&self) -> &'static str {
        "Streamlabs OBS"
    }
} 