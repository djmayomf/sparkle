use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TailSystem {
    pub segments: Vec<TailSegment>,
    pub base_position: (f32, f32, f32),
    pub current_motion: TailMotion,
    pub emotion_influence: f32,
    pub physics_params: TailPhysics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TailSegment {
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub length: f32,
    pub weight: f32,
    pub constraints: TailConstraints,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TailConstraints {
    pub max_angle: f32,
    pub stiffness: f32,
    pub damping: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TailMotion {
    Idle,
    Excited,
    Alert,
    Focused,
    Hacking,  // Special motion for cybersecurity activities
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TailPhysics {
    pub gravity: f32,
    pub wind_influence: f32,
    pub momentum: f32,
    pub spring_constant: f32,
}

impl TailSystem {
    pub fn new() -> Self {
        Self {
            segments: Self::initialize_segments(),
            base_position: (0.0, 0.0, 0.0),
            current_motion: TailMotion::Idle,
            emotion_influence: 0.5,
            physics_params: TailPhysics {
                gravity: 9.81,
                wind_influence: 0.1,
                momentum: 0.8,
                spring_constant: 0.5,
            },
        }
    }

    fn initialize_segments() -> Vec<TailSegment> {
        // Create 5 segments for smooth motion
        (0..5).map(|i| {
            TailSegment {
                position: (0.0, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0),
                length: 0.2 - (i as f32 * 0.03), // Gradually decrease length
                weight: 1.0 - (i as f32 * 0.15), // Gradually decrease weight
                constraints: TailConstraints {
                    max_angle: 45.0,
                    stiffness: 0.8 - (i as f32 * 0.1),
                    damping: 0.3,
                },
            }
        }).collect()
    }

    pub fn update(&mut self, delta_time: f32, activity: &str) {
        match activity {
            "ctf_solving" => {
                self.current_motion = TailMotion::Hacking;
                self.emotion_influence = 0.8;
            },
            "flag_found" => {
                self.current_motion = TailMotion::Excited;
                self.emotion_influence = 1.0;
            },
            _ => {
                self.current_motion = TailMotion::Idle;
                self.emotion_influence = 0.5;
            }
        }

        self.apply_physics(delta_time);
    }

    fn apply_physics(&mut self, delta_time: f32) {
        // Apply physics calculations to each segment
        for i in 0..self.segments.len() {
            // Physics simulation code here
        }
    }
} 