pub struct GameAIIntegrator {
    personality_core: Arc<RwLock<PersonalityCore>>,
    game_ai: Arc<GameAI>,
    commentary_system: Arc<CommentaryAIBrain>,
    
    pub async fn integrate_systems(&mut self) -> Result<()> {
        // Sync personality with game AI
        let personality = self.personality_core.read().await?;
        
        // Adapt AI behavior based on personality
        self.game_ai.player_behavior.adapt_to_personality(&personality);
        self.commentary_system.personality_core.adapt_to_game_situation(
            self.game_ai.get_current_context()
        );
        
        Ok(())
    }
} 