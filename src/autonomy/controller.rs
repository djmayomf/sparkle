use crate::error::Result;
use crate::emotions::adapter::EmotionalAdapter;
use crate::ai::neural_chat::NeuralChat;
use crate::memory::cache::MemoryCache;
use std::sync::Arc;

pub struct AutonomyController {
    emotional_adapter: Arc<tokio::sync::RwLock<EmotionalAdapter>>,
    neural_chat: Arc<NeuralChat>,
    memory_cache: Arc<MemoryCache>,
}

impl AutonomyController {
    pub async fn new(
        emotional_adapter: Arc<tokio::sync::RwLock<EmotionalAdapter>>,
        neural_chat: Arc<NeuralChat>,
        memory_cache: Arc<MemoryCache>,
    ) -> Result<Self> {
        Ok(Self {
            emotional_adapter,
            neural_chat,
            memory_cache,
        })
    }
} 