use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct NaturalBehaviorSystem {
    pub idle_behaviors: HashMap<String, IdleBehavior>,
    pub current_state: StreamState,
    pub behavior_queue: Vec<BehaviorTrigger>,
    pub comfort_movements: ComfortMovements,
    pub attention_system: AttentionSystem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdleBehavior {
    pub name: String,
    pub frequency: f32,
    pub duration: f32,
    pub movement_set: Vec<MovementKeyframe>,
    pub interrupt_priority: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamState {
    pub energy_level: f32,
    pub comfort_level: f32,
    pub attention_focus: (f32, f32), // x, y coordinates of attention
    pub last_movement: f32, // timestamp
    pub current_posture: PostureState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComfortMovements {
    pub weight_shifts: Vec<WeightShift>,
    pub stretches: Vec<StretchMovement>,
    pub fidgets: Vec<FidgetMovement>,
    pub breathing_pattern: BreathingPattern,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttentionSystem {
    pub focus_point: (f32, f32),
    pub distractions: Vec<Distraction>,
    pub attention_span: f32,
    pub interest_areas: Vec<InterestArea>,
}

impl NaturalBehaviorSystem {
    pub fn new() -> Self {
        Self {
            idle_behaviors: Self::initialize_idle_behaviors(),
            current_state: StreamState::default(),
            behavior_queue: Vec::new(),
            comfort_movements: ComfortMovements::default(),
            attention_system: AttentionSystem::default(),
        }
    }

    fn initialize_idle_behaviors() -> HashMap<String, IdleBehavior> {
        let mut behaviors = HashMap::new();
        
        // Natural sitting position adjustments
        behaviors.insert(
            "weight_shift".to_string(),
            IdleBehavior {
                name: "Weight Shift".to_string(),
                frequency: 0.2,
                duration: 2.0,
                movement_set: vec![],
                interrupt_priority: 1,
            }
        );

        // Subtle head tilts when reading chat
        behaviors.insert(
            "chat_reading".to_string(),
            IdleBehavior {
                name: "Read Chat".to_string(),
                frequency: 0.4,
                duration: 1.5,
                movement_set: vec![],
                interrupt_priority: 2,
            }
        );

        behaviors
    }

    pub fn update_behavior(&mut self, timestamp: f32) -> Vec<(String, f32)> {
        let mut params = Vec::new();
        
        // Update breathing
        self.update_breathing(timestamp, &mut params);
        
        // Update comfort movements
        self.update_comfort_movements(timestamp, &mut params);
        
        // Update attention system
        self.update_attention(timestamp, &mut params);
        
        params
    }

    fn update_breathing(&mut self, timestamp: f32, params: &mut Vec<(String, f32)>) {
        // Implement natural breathing cycle
        let breath_cycle = (timestamp * 0.2).sin() * 0.5 + 0.5;
        params.push(("ParamBreathingCycle".to_string(), breath_cycle));
    }

    fn update_comfort_movements(&mut self, timestamp: f32, params: &mut Vec<(String, f32)>) {
        // Add subtle weight shifts and posture adjustments
        if timestamp - self.current_state.last_movement > 30.0 {
            self.trigger_comfort_movement(params);
        }
    }

    fn update_attention(&mut self, timestamp: f32, params: &mut Vec<(String, f32)>) {
        // Implement natural attention patterns
        // Look at chat, then back to center, occasional glances around
    }
} 