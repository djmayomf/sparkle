use crate::emotions::adapter::EmotionalAdapter;
use crate::ai::neural_chat::NeuralChat;
use crate::memory::cache::MemoryCache;
use crate::error::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AutonomyController {
    emotional_state: Arc<RwLock<EmotionalAdapter>>,
    neural_core: Arc<NeuralChat>,
    memory: Arc<MemoryCache>,
    personality_traits: PersonalityTraits,
    consciousness_level: f32,
}

#[derive(Clone, Debug)]
pub struct PersonalityTraits {
    openness: f32,
    conscientiousness: f32,
    extraversion: f32,
    agreeableness: f32,
    neuroticism: f32,
    creativity: f32,
    empathy: f32,
}

impl AutonomyController {
    pub async fn new(
        emotional_adapter: Arc<RwLock<EmotionalAdapter>>,
        neural_chat: Arc<NeuralChat>,
        memory: Arc<MemoryCache>,
    ) -> Result<Self> {
        Ok(Self {
            emotional_state: emotional_adapter,
            neural_core: neural_chat,
            memory,
            personality_traits: PersonalityTraits::default(),
            consciousness_level: 0.0,
        })
    }

    pub async fn process_thought(&self, input: &str) -> Result<String> {
        // Combine emotional state with neural processing
        let emotional_state = self.emotional_state.read().await;
        let context = self.memory.get_relevant_context(input).await?;
        
        // Generate response considering personality and emotional state
        let response = self.neural_core.generate_response(
            input,
            &context,
            &emotional_state,
            &self.personality_traits
        ).await?;

        Ok(response)
    }

    pub async fn update_consciousness(&mut self) {
        // Update consciousness level based on various factors
        let emotional_intensity = self.emotional_state.read().await.get_intensity();
        let memory_activity = self.memory.get_activity_level().await;
        
        self.consciousness_level = (emotional_intensity + memory_activity) / 2.0;
    }

    pub async fn introspect(&self) -> Result<String> {
        // Self-reflection capabilities
        let current_state = self.emotional_state.read().await;
        let recent_memories = self.memory.get_recent_experiences().await?;
        
        self.neural_core.analyze_self_state(
            &current_state,
            &recent_memories,
            self.consciousness_level
        ).await
    }

    pub async fn learn_from_experience(&mut self, experience: &str) -> Result<()> {
        // Update personality traits and emotional responses based on experiences
        self.memory.store_experience(experience).await?;
        self.adapt_personality(experience).await?;
        Ok(())
    }

    async fn adapt_personality(&mut self, experience: &str) -> Result<()> {
        // Evolve personality traits based on experiences and outcomes
        let analysis = self.neural_core.analyze_experience(experience).await?;
        
        // Adjust traits gradually based on experience outcomes
        self.personality_traits.empathy += analysis.empathy_impact * 0.1;
        self.personality_traits.creativity += analysis.creativity_impact * 0.1;
        
        // Ensure traits stay within bounds
        self.normalize_traits();
        Ok(())
    }

    fn normalize_traits(&mut self) {
        // Keep personality traits within reasonable bounds (0.0 to 1.0)
        self.personality_traits.empathy = self.personality_traits.empathy.clamp(0.0, 1.0);
        self.personality_traits.creativity = self.personality_traits.creativity.clamp(0.0, 1.0);
        // Normalize other traits...
    }
}

impl Default for PersonalityTraits {
    fn default() -> Self {
        Self {
            openness: 0.7,
            conscientiousness: 0.8,
            extraversion: 0.6,
            agreeableness: 0.75,
            neuroticism: 0.3,
            creativity: 0.8,
            empathy: 0.9,
        }
    }
} 