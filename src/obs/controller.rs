use obws::{Client as OBSClient, requests::scenes::SetCurrentScene};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneConfig {
    pub name: String,
    pub elements: Vec<String>,
    pub transitions: HashMap<String, String>, // target_scene -> transition_type
}

pub struct OBSController {
    client: OBSClient,
    scenes: HashMap<String, SceneConfig>,
    current_scene: String,
}

impl OBSController {
    pub async fn new(host: String, port: u16, password: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let client = OBSClient::connect(format!("ws://{}:{}", host, port), password).await?;
        
        Ok(Self {
            client,
            scenes: HashMap::new(),
            current_scene: String::new(),
        })
    }

    pub async fn switch_scene(&mut self, scene_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(scene_config) = self.scenes.get(scene_name) {
            let transition = self.scenes
                .get(&self.current_scene)
                .and_then(|current| current.transitions.get(scene_name))
                .unwrap_or(&"Cut".to_string());

            self.client.send(SetCurrentScene {
                name: scene_name.to_string(),
            }).await?;

            self.current_scene = scene_name.to_string();
            Ok(())
        } else {
            Err("Scene not found".into())
        }
    }

    pub async fn add_scene(&mut self, config: SceneConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.scenes.insert(config.name.clone(), config);
        Ok(())
    }

    pub fn get_current_scene(&self) -> &str {
        &self.current_scene
    }
} 