use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DanceRigSystem {
    pub dance_moves: HashMap<String, DanceMove>,
    pub choreographies: HashMap<String, Choreography>,
    pub current_dance: Option<String>,
    pub transition_state: TransitionState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DanceMove {
    pub name: String,
    pub move_type: MoveType,
    pub keyframes: Vec<DanceKeyframe>,
    pub required_params: Vec<String>,
    pub difficulty: DanceDifficulty,
    pub energy_level: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MoveType {
    Point,           // Point dance moves
    Formation,       // Group formation positions
    Transition,      // Smooth transitions between moves
    FloorWork,      // Floor-based choreography
    Jump,           // Jumping sequences
    Spin,           // Spinning moves
    Isolation,      // Isolated body part movements
    Wave,           // Wave motions
    Choreography,   // Choreography moves
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DanceKeyframe {
    pub timestamp: f32,
    pub body_position: BodyPosition,
    pub expression: String,
    pub energy: f32,
    pub interpolation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BodyPosition {
    pub head: (f32, f32, f32),
    pub torso: (f32, f32, f32),
    pub arms: ArmPosition,
    pub legs: LegPosition,
    pub spine_curve: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmPosition {
    pub left: Vec<JointAngle>,
    pub right: Vec<JointAngle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegPosition {
    pub left: Vec<JointAngle>,
    pub right: Vec<JointAngle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JointAngle {
    pub joint: String,
    pub rotation: (f32, f32, f32),
    pub constraint: Option<AngleConstraint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AngleConstraint {
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choreography {
    pub song_name: String,
    pub artist: String,
    pub bpm: f32,
    pub sequence: Vec<ChoreographySegment>,
    pub difficulty: DanceDifficulty,
    pub energy_curve: Vec<(f32, f32)>, // timestamp, energy level
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChoreographySegment {
    pub start_time: f32,
    pub end_time: f32,
    pub moves: Vec<String>,
    pub formation_position: Option<(f32, f32)>,
    pub transition_type: TransitionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransitionType {
    Smooth,
    Sharp,
    Flow,
    Beat,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DanceDifficulty {
    Beginner,
    Intermediate,
    Advanced,
    Master,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionState {
    pub current_move: Option<String>,
    pub next_move: Option<String>,
    pub progress: f32,
    pub blend_factor: f32,
}

impl DanceRigSystem {
    pub fn new() -> Self {
        Self {
            dance_moves: Self::initialize_dance_moves(),
            choreographies: Self::initialize_choreographies(),
            current_dance: None,
            transition_state: TransitionState {
                current_move: None,
                next_move: None,
                progress: 0.0,
                blend_factor: 0.0,
            },
        }
    }

    fn initialize_dance_moves() -> HashMap<String, DanceMove> {
        let mut moves = HashMap::new();
        
        // Popular K-pop point moves
        moves.insert(
            "heart_point".to_string(),
            DanceMove {
                name: "Heart Point".to_string(),
                move_type: MoveType::Point,
                keyframes: vec![],
                required_params: vec!["arms".to_string(), "hands".to_string()],
                difficulty: DanceDifficulty::Beginner,
                energy_level: 0.3,
            }
        );

        // Add more signature moves
        moves.insert(
            "wave_body_roll".to_string(),
            DanceMove {
                name: "Wave Body Roll".to_string(),
                move_type: MoveType::Wave,
                keyframes: vec![],
                required_params: vec!["spine".to_string(), "shoulders".to_string()],
                difficulty: DanceDifficulty::Intermediate,
                energy_level: 0.6,
            }
        );

        // Touch by Katseye specific moves
        moves.insert(
            "touch_main_sequence".to_string(),
            DanceMove {
                name: "Touch Main Sequence".to_string(),
                move_type: MoveType::Choreography,
                keyframes: vec![
                    DanceKeyframe {
                        timestamp: 0.0,
                        body_position: BodyPosition::default(),
                        expression: "focused".to_string(),
                        energy: 0.8,
                        interpolation: "smooth".to_string(),
                    },
                    // Add more keyframes for the full sequence
                ],
                required_params: vec![
                    "full_body".to_string(),
                    "arms".to_string(),
                    "expression".to_string()
                ],
                difficulty: DanceDifficulty::Advanced,
                energy_level: 0.9,
            }
        );

        // Add Touch-specific movements
        moves.insert(
            "touch_wave".to_string(),
            DanceMove {
                name: "Touch Wave Sequence".to_string(),
                move_type: MoveType::Wave,
                keyframes: vec![],
                required_params: vec!["body_wave".to_string()],
                difficulty: DanceDifficulty::Advanced,
                energy_level: 0.85,
            }
        );

        moves
    }

    fn initialize_choreographies() -> HashMap<String, Choreography> {
        let mut choreos = HashMap::new();

        // Example choreographies
        choreos.insert(
            "dynamite".to_string(),
            Choreography {
                song_name: "Dynamite".to_string(),
                artist: "BTS".to_string(),
                bpm: 114.0,
                sequence: vec![],
                difficulty: DanceDifficulty::Intermediate,
                energy_curve: vec![(0.0, 0.5), (30.0, 0.8)],
            }
        );

        // Add more choreographies
        choreos.insert(
            "how_you_like_that".to_string(),
            Choreography {
                song_name: "How You Like That".to_string(),
                artist: "BLACKPINK".to_string(),
                bpm: 130.0,
                sequence: vec![],
                difficulty: DanceDifficulty::Advanced,
                energy_curve: vec![(0.0, 0.6), (45.0, 0.9)],
            }
        );

        // Touch by Katseye
        choreos.insert(
            "touch".to_string(),
            Choreography {
                song_name: "Touch".to_string(),
                artist: "Katseye".to_string(),
                bpm: 128.0,
                sequence: vec![
                    ChoreographySegment {
                        start_time: 0.0,
                        end_time: 15.0,
                        moves: vec!["touch_intro".to_string()],
                        formation_position: Some((0.0, 0.0)),
                        transition_type: TransitionType::Flow,
                    },
                    ChoreographySegment {
                        start_time: 15.0,
                        end_time: 30.0,
                        moves: vec!["touch_main_sequence".to_string()],
                        formation_position: Some((0.0, 0.0)),
                        transition_type: TransitionType::Sharp,
                    },
                    // Add more segments for the full song
                ],
                difficulty: DanceDifficulty::Advanced,
                energy_curve: vec![
                    (0.0, 0.6),   // Intro
                    (15.0, 0.8),  // Build-up
                    (30.0, 0.9),  // Chorus
                    (45.0, 0.85), // Verse
                    // Continue energy curve
                ],
            }
        );

        choreos
    }

    pub fn toggle_dance(&mut self, choreo_name: &str) -> Result<(), String> {
        if self.choreographies.contains_key(choreo_name) {
            self.current_dance = Some(choreo_name.to_string());
            Ok(())
        } else {
            Err("Choreography not found".to_string())
        }
    }

    pub fn update_dance(&mut self, timestamp: f32) -> Vec<(String, f32)> {
        let mut params = Vec::new();
        
        if let Some(dance_name) = &self.current_dance {
            if let Some(choreo) = self.choreographies.get(dance_name) {
                // Calculate current position in choreography
                // Generate Live2D parameters for current move
                // Handle transitions between moves
            }
        }
        
        params
    }
} 