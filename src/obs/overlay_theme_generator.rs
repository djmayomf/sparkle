use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::scene_generator::OverlayConfig;
use crate::model::design_spec::{ColorPalette, CyberPartsSpec};

#[derive(Debug, Serialize, Deserialize)]
pub struct OverlayThemeGenerator {
    pub current_theme: ThemeConfig,
    pub color_scheme: ColorScheme,
    pub animation_sets: HashMap<String, AnimationSet>,
    pub element_styles: ElementStyles,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
    pub glow_color: String,
    pub background_style: BackgroundStyle,
    pub font_family: String,
    pub border_style: BorderStyle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorScheme {
    pub base_colors: Vec<String>,
    pub gradient_maps: HashMap<String, GradientConfig>,
    pub glow_effects: HashMap<String, GlowEffect>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementStyles {
    pub alerts: AlertStyle,
    pub chat_box: ChatBoxStyle,
    pub info_panels: InfoPanelStyle,
    pub transitions: TransitionStyle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertStyle {
    pub border_radius: f32,
    pub glow_intensity: f32,
    pub animation_style: String,
    pub background_opacity: f32,
}

impl OverlayThemeGenerator {
    pub fn new(model_colors: &ColorPalette, cyber_parts: &CyberPartsSpec) -> Self {
        Self {
            current_theme: Self::generate_theme_from_model(model_colors, cyber_parts),
            color_scheme: Self::create_color_scheme(model_colors),
            animation_sets: Self::initialize_animations(cyber_parts),
            element_styles: Self::create_element_styles(model_colors),
        }
    }

    fn generate_theme_from_model(colors: &ColorPalette, cyber_parts: &CyberPartsSpec) -> ThemeConfig {
        ThemeConfig {
            primary_color: colors.accent_lights.clone(),    // Cyber teal
            secondary_color: colors.jacket_base.clone(),    // Dark charcoal
            accent_color: colors.jacket_trim.clone(),       // Pink accent
            glow_color: colors.eyes.clone(),               // Bright teal
            background_style: BackgroundStyle::CyberGrid {
                color: colors.accent_lights.clone(),
                opacity: 0.15,
                animation_speed: 1.0,
            },
            font_family: "Cyberpunk".to_string(),
            border_style: BorderStyle::Cyber {
                width: 2.0,
                glow: true,
            },
        }
    }

    pub async fn generate_overlay_set(&self, scene_type: &str) -> HashMap<String, OverlayConfig> {
        let mut overlays = HashMap::new();

        match scene_type {
            "gaming" => {
                overlays.insert("game_info".to_string(), self.create_game_info_overlay());
                overlays.insert("alerts".to_string(), self.create_gaming_alerts());
                overlays.insert("chat".to_string(), self.create_gaming_chat());
            },
            "just_chatting" => {
                overlays.insert("main_frame".to_string(), self.create_chat_frame());
                overlays.insert("alerts".to_string(), self.create_chat_alerts());
                overlays.insert("info_panel".to_string(), self.create_info_panel());
            },
            "ctf_solving" => {
                overlays.insert("terminal".to_string(), self.create_terminal_overlay());
                overlays.insert("progress".to_string(), self.create_ctf_progress());
                overlays.insert("tools".to_string(), self.create_tools_panel());
            },
            _ => {}
        }

        overlays
    }

    fn create_terminal_overlay(&self) -> OverlayConfig {
        OverlayConfig::Custom {
            name: "Cyber Terminal".to_string(),
            style: OverlayStyle {
                background_color: self.current_theme.secondary_color.clone(),
                border: BorderStyle::Cyber {
                    width: 2.0,
                    glow: true,
                },
                glow_color: self.current_theme.glow_color.clone(),
                opacity: 0.9,
                animations: vec![
                    "scan_line".to_string(),
                    "data_flow".to_string(),
                ],
            },
            position: (0.1, 0.1),
            size: (0.8, 0.8),
        }
    }

    pub async fn update_theme_from_model(&mut self, model_update: &ModelUpdate) -> Result<(), String> {
        // Update theme based on model changes
        if let Some(new_colors) = &model_update.color_changes {
            self.update_color_scheme(new_colors);
        }

        if let Some(new_cyber) = &model_update.cyber_changes {
            self.update_animations(new_cyber);
        }

        // Generate new overlays with updated theme
        self.regenerate_active_overlays().await?;

        Ok(())
    }

    async fn regenerate_active_overlays(&self) -> Result<(), String> {
        // Regenerate all active overlays with new theme
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverlayStyle {
    pub background_color: String,
    pub border: BorderStyle,
    pub glow_color: String,
    pub opacity: f32,
    pub animations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BorderStyle {
    None,
    Simple { width: f32 },
    Cyber { width: f32, glow: bool },
    Animated { width: f32, pattern: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BackgroundStyle {
    Solid { color: String },
    Gradient { colors: Vec<String> },
    CyberGrid { color: String, opacity: f32, animation_speed: f32 },
    Animated { pattern: String, colors: Vec<String> },
} 