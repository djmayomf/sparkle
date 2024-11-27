use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::overlay_theme_generator::OverlayThemeGenerator;
use crate::model::design_spec::{ColorPalette, CyberPartsSpec};

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneGenerator {
    pub templates: HashMap<SceneType, SceneTemplate>,
    pub active_scene: Option<String>,
    pub overlay_manager: OverlayManager,
    pub transition_effects: TransitionEffects,
    pub theme_generator: OverlayThemeGenerator,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum SceneType {
    JustChatting,
    Gaming {
        game_name: String,
        platform: String,
    },
    YouTube {
        video_type: String,
        reaction_mode: bool,
    },
    CTFSolving,
    DancePerformance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneTemplate {
    pub name: String,
    pub layout: SceneLayout,
    pub overlays: Vec<OverlayConfig>,
    pub animations: Vec<AnimationConfig>,
    pub alerts: AlertConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverlayManager {
    pub active_overlays: Vec<ActiveOverlay>,
    pub custom_elements: HashMap<String, OverlayElement>,
    pub alert_queue: Vec<AlertEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneLayout {
    pub model_position: (f32, f32),
    pub model_scale: f32,
    pub camera_angles: CameraAngles,
    pub background: BackgroundConfig,
    pub chat_position: Option<(f32, f32)>,
    pub content_area: Option<ContentArea>,
}

impl SceneGenerator {
    pub fn new(model_colors: &ColorPalette, cyber_parts: &CyberPartsSpec) -> Self {
        Self {
            templates: Self::initialize_templates(),
            active_scene: None,
            overlay_manager: OverlayManager::new(),
            transition_effects: TransitionEffects::default(),
            theme_generator: OverlayThemeGenerator::new(model_colors, cyber_parts),
        }
    }

    fn initialize_templates() -> HashMap<SceneType, SceneTemplate> {
        let mut templates = HashMap::new();

        // Just Chatting Scene
        templates.insert(
            SceneType::JustChatting,
            SceneTemplate {
                name: "Just Chatting".to_string(),
                layout: SceneLayout {
                    model_position: (0.7, 0.5),  // Right side of screen
                    model_scale: 1.0,
                    camera_angles: CameraAngles::default(),
                    background: BackgroundConfig::Animated {
                        theme: "cyber_lofi".to_string(),
                        intensity: 0.7,
                    },
                    chat_position: Some((0.1, 0.5)),  // Left side
                    content_area: None,
                },
                overlays: vec![
                    OverlayConfig::ChatBox {
                        transparency: 0.8,
                        theme: "cyber_neon".to_string(),
                    },
                    OverlayConfig::EventList {
                        position: (0.05, 0.1),
                        max_items: 5,
                    },
                ],
                animations: vec![],
                alerts: AlertConfig::default(),
            }
        );

        // Gaming Scene
        templates.insert(
            SceneType::Gaming {
                game_name: "default".to_string(),
                platform: "default".to_string(),
            },
            SceneTemplate {
                name: "Gaming Layout".to_string(),
                layout: SceneLayout {
                    model_position: (0.8, 0.3),  // Top right corner
                    model_scale: 0.7,
                    camera_angles: CameraAngles::default(),
                    background: BackgroundConfig::Game,
                    chat_position: Some((0.8, 0.7)),  // Bottom right
                    content_area: Some(ContentArea {
                        position: (0.0, 0.0),
                        size: (0.75, 1.0),  // Game takes up most of screen
                    }),
                },
                overlays: vec![
                    OverlayConfig::GameInfo {
                        position: (0.05, 0.05),
                        show_stats: true,
                    },
                ],
                animations: vec![],
                alerts: AlertConfig::default(),
            }
        );

        // YouTube Scene
        templates.insert(
            SceneType::YouTube {
                video_type: "reaction".to_string(),
                reaction_mode: true,
            },
            SceneTemplate {
                name: "YouTube Reaction".to_string(),
                layout: SceneLayout {
                    model_position: (0.7, 0.7),  // Bottom right
                    model_scale: 0.6,
                    camera_angles: CameraAngles::default(),
                    background: BackgroundConfig::Dark,
                    chat_position: Some((0.7, 0.3)),  // Top right
                    content_area: Some(ContentArea {
                        position: (0.0, 0.0),
                        size: (0.65, 1.0),  // Video on left side
                    }),
                },
                overlays: vec![
                    OverlayConfig::VideoInfo {
                        position: (0.05, 0.05),
                        show_title: true,
                    },
                ],
                animations: vec![],
                alerts: AlertConfig::default(),
            }
        );

        templates
    }

    pub async fn switch_scene(&mut self, scene_type: SceneType) -> Result<(), String> {
        if let Some(template) = self.templates.get(&scene_type) {
            self.active_scene = Some(template.name.clone());
            self.overlay_manager.clear_overlays();
            self.apply_scene_template(template).await?;
        }
        Ok(())
    }

    async fn apply_scene_template(&mut self, template: &SceneTemplate) -> Result<(), String> {
        // Apply layout
        self.apply_layout(&template.layout).await?;
        
        // Generate and apply overlays
        for overlay in &template.overlays {
            self.overlay_manager.add_overlay(overlay.clone()).await?;
        }
        
        // Apply animations
        for animation in &template.animations {
            self.apply_animation(animation).await?;
        }
        
        Ok(())
    }

    async fn apply_layout(&self, layout: &SceneLayout) -> Result<(), String> {
        // Set model position and scale
        // Configure camera angles
        // Set up background
        // Position chat and content areas
        Ok(())
    }

    async fn apply_animation(&self, animation: &AnimationConfig) -> Result<(), String> {
        // Apply scene transition animations
        Ok(())
    }

    pub async fn update_theme_from_model(&mut self, model_update: &ModelUpdate) -> Result<(), String> {
        // Update theme generator with new model aesthetics
        self.theme_generator.update_theme_from_model(model_update).await?;
        
        // Regenerate overlays for active scene if needed
        if let Some(active) = &self.active_scene {
            self.regenerate_scene_overlays(active).await?;
        }
        
        Ok(())
    }

    async fn regenerate_scene_overlays(&mut self, scene_name: &str) -> Result<(), String> {
        let overlays = self.theme_generator.generate_overlay_set(scene_name).await;
        
        // Update active overlays with new themed versions
        self.overlay_manager.update_overlays(overlays).await?;
        
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OverlayConfig {
    ChatBox {
        transparency: f32,
        theme: String,
    },
    EventList {
        position: (f32, f32),
        max_items: usize,
    },
    GameInfo {
        position: (f32, f32),
        show_stats: bool,
    },
    VideoInfo {
        position: (f32, f32),
        show_title: bool,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveOverlay {
    pub id: String,
    pub config: OverlayConfig,
    pub state: OverlayState,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OverlayState {
    Visible,
    Hidden,
    Transitioning(f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertConfig {
    pub position: (f32, f32),
    pub duration: f32,
    pub animation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertEvent {
    pub event_type: String,
    pub message: String,
    pub duration: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BackgroundConfig {
    Static { color: String },
    Animated { theme: String, intensity: f32 },
    Game,
    Dark,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentArea {
    pub position: (f32, f32),
    pub size: (f32, f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraAngles {
    pub rotation: (f32, f32, f32),
    pub zoom: f32,
}

impl Default for CameraAngles {
    fn default() -> Self {
        Self {
            rotation: (0.0, 0.0, 0.0),
            zoom: 1.0,
        }
    }
} 