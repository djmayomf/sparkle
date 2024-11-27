use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::dance_rigging::DanceRigSystem;

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorPalette {
    // Hair colors
    pub hair_root: String,      // #4B0082 (Dark indigo)
    pub hair_mid: String,       // #8A2BE2 (Purple)
    pub hair_tips: String,      // #FF69B4 (Pink gradient)
    
    // Skin and features
    pub skin_tone: String,      // #8D5524 (Rich mocha)
    pub eyes: String,           // #00FFD1 (Bright teal)
    pub lips: String,           // #990066 (Deep rose)
    
    // Outfit colors
    pub jacket_base: String,    // #1A1A1A (Dark charcoal)
    pub jacket_trim: String,    // #FF1493 (Pink accent)
    pub pants_base: String,     // #1A237E (Navy blue)
    pub accent_lights: String,  // #00FFD1 (Cyber teal)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HornSpec {
    pub base_color: String,     // #1A1A1A (Black)
    pub gradient_start: String, // #FF4500 (Orange)
    pub gradient_end: String,   // #FF0000 (Red)
    pub metallic_factor: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CyberEnhancement {
    pub location: String,
    pub glow_color: String,
    pub animation_params: HashMap<String, f32>,
    pub power_state: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutfitSpec {
    // Jacket
    pub jacket: JacketSpec,
    // Top
    pub crop_top: CropTopSpec,
    // Pants
    pub tactical_pants: TacticalPantsSpec,
    // Accessories
    pub accessories: Vec<AccessorySpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JacketSpec {
    pub style: String,          // "cyber_cropped"
    pub base_color: String,     // #1A1A1A
    pub trim_color: String,     // #FF1493
    pub light_panels: Vec<LightPanel>,
    pub collar_style: String,   // "standing"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CropTopSpec {
    pub text: String,           // "VATEN"
    pub base_color: String,     // #000000
    pub text_color: String,     // #FFFFFF
    pub fit: String,           // "tight"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TacticalPantsSpec {
    pub base_color: String,     // #1A237E
    pub straps: Vec<StrapSpec>,
    pub light_strips: Vec<LightStrip>,
    pub holsters: Vec<HolsterSpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightPanel {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: String,
    pub glow_intensity: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrapSpec {
    pub position: (f32, f32),
    pub length: f32,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightStrip {
    pub position: (f32, f32),
    pub length: f32,
    pub color: String,
    pub animation_pattern: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HolsterSpec {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessorySpec {
    pub name: String,
    pub type_: String,
    pub color: String,
    pub glow_enabled: bool,
    pub animation_params: Option<HashMap<String, f32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullModelSpec {
    pub colors: ColorPalette,
    pub body: BodySpec,
    pub face: FaceSpec,
    pub hair: HairSpec,
    pub horns: HornSpec,
    pub cyber_parts: CyberPartsSpec,
    pub outfit: OutfitSpec,
    pub physics: PhysicsSpec,
    pub expressions: ExpressionSpec,
    pub dance_system: DanceRigSystem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BodySpec {
    pub height: f32,          // Standard height units
    pub proportions: Proportions,
    pub pose: PoseSpec,
    pub skin_properties: SkinProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proportions {
    pub head_to_body_ratio: f32,
    pub shoulder_width: f32,
    pub waist: f32,
    pub hip: f32,
    pub leg_length: f32,
    pub arm_length: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoseSpec {
    pub default_pose: String,
    pub spine_curve: f32,
    pub shoulder_rotation: (f32, f32, f32),
    pub hip_rotation: (f32, f32, f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkinProperties {
    pub base_tone: String,     // #8D5524
    pub highlight_tone: String,
    pub shadow_tone: String,
    pub texture_map: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaceSpec {
    pub eye_shape: EyeShape,
    pub nose: NoseSpec,
    pub mouth: MouthSpec,
    pub face_shape: String,
    pub expressions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EyeShape {
    pub size: (f32, f32),
    pub angle: f32,
    pub color: String,         // #00FFD1
    pub highlight_pattern: String,
    pub cyber_effects: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HairSpec {
    pub length: f32,
    pub volume: f32,
    pub gradient_colors: Vec<String>,
    pub physics_points: Vec<(f32, f32)>,
    pub style: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CyberPartsSpec {
    pub enhancements: Vec<CyberEnhancement>,
    pub glow_patterns: Vec<GlowPattern>,
    pub animation_sets: HashMap<String, Vec<AnimationKeyframe>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlowPattern {
    pub pattern_type: String,
    pub color: String,
    pub intensity: f32,
    pub pulse_rate: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationKeyframe {
    pub timestamp: f32,
    pub value: f32,
    pub interpolation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsSpec {
    pub hair_physics: HairPhysics,
    pub cloth_physics: ClothPhysics,
    pub accessory_physics: AccessoryPhysics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HairPhysics {
    pub gravity: f32,
    pub wind_influence: f32,
    pub bounce: f32,
    pub segments: Vec<PhysicsSegment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsSegment {
    pub weight: f32,
    pub damping: f32,
    pub stiffness: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpressionSpec {
    pub base_expressions: Vec<String>,
    pub blendshapes: Vec<BlendShape>,
    pub emotion_mappings: HashMap<String, Vec<BlendShape>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlendShape {
    pub name: String,
    pub intensity: f32,
    pub affected_areas: Vec<String>,
}

impl FullModelSpec {
    pub fn new() -> Self {
        Self {
            colors: ColorPalette::default(),
            body: BodySpec::default(),
            face: FaceSpec::default(),
            hair: HairSpec::default(),
            horns: HornSpec::default(),
            cyber_parts: CyberPartsSpec::default(),
            outfit: OutfitSpec::default(),
            physics: PhysicsSpec::default(),
            expressions: ExpressionSpec::default(),
            dance_system: DanceRigSystem::new(),
        }
    }

    pub fn generate_live2d_params(&self) -> HashMap<String, f32> {
        let mut params = HashMap::new();
        
        // Core deformation parameters
        params.insert("ParamBodyAngleX".to_string(), 0.0);
        params.insert("ParamBodyAngleY".to_string(), 0.0);
        params.insert("ParamBodyAngleZ".to_string(), 0.0);

        // Breathing animation
        params.insert("ParamBreath".to_string(), 0.0);
        
        // Face tracking
        params.insert("ParamAngleX".to_string(), 0.0);
        params.insert("ParamAngleY".to_string(), 0.0);
        params.insert("ParamAngleZ".to_string(), 0.0);
        
        // Eye tracking and blinking
        params.insert("ParamEyeLOpen".to_string(), 1.0);
        params.insert("ParamEyeROpen".to_string(), 1.0);
        params.insert("ParamEyeBallX".to_string(), 0.0);
        params.insert("ParamEyeBallY".to_string(), 0.0);
        
        // Mouth and expressions
        params.insert("ParamMouthForm".to_string(), 0.0);
        params.insert("ParamMouthOpenY".to_string(), 0.0);
        
        // Hair physics
        for i in 0..self.physics.hair_physics.segments.len() {
            params.insert(format!("ParamHairFront{}", i), 0.0);
            params.insert(format!("ParamHairSide{}", i), 0.0);
            params.insert(format!("ParamHairBack{}", i), 0.0);
        }
        
        // Cyber enhancements
        for (i, enhancement) in self.cyber_parts.enhancements.iter().enumerate() {
            params.insert(format!("ParamCyberGlow{}", i), 0.8);
            params.insert(format!("ParamCyberPulse{}", i), 0.0);
        }
        
        // Cloth physics
        for i in 0..self.physics.cloth_physics.segments.len() {
            params.insert(format!("ParamCloth{}", i), 0.0);
        }
        
        // Add dance parameters if active
        if let Some(dance_params) = self.generate_dance_params() {
            params.extend(dance_params);
        }
        
        params
    }

    fn generate_dance_params(&self) -> Option<HashMap<String, f32>> {
        if let Some(current_dance) = &self.dance_system.current_dance {
            let mut dance_params = HashMap::new();
            
            // Add dance-specific parameters
            dance_params.insert("ParamDanceEnergy".to_string(), 0.0);
            dance_params.insert("ParamDanceFlow".to_string(), 0.0);
            dance_params.insert("ParamDanceExpression".to_string(), 0.0);
            
            Some(dance_params)
        } else {
            None
        }
    }
} 