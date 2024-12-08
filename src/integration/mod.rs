use crate::ai::PersonalityCore;
use crate::streaming::system_orchestrator::SystemOrchestrator;
use crate::content::creator::ContentCreator;
use crate::obs::scene_generator::SceneGenerator;
use crate::vrchat::controller::VRChatController;

pub struct SystemIntegrator {
    personality_core: Arc<RwLock<PersonalityCore>>,
    system_orchestrator: Arc<RwLock<SystemOrchestrator>>,
    content_creator: Arc<ContentCreator>,
    scene_generator: Arc<SceneGenerator>,
    vrchat_controller: Option<Arc<VRChatController>>,
}

impl SystemIntegrator {
    pub async fn synchronize_systems(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Add performance monitoring
        let start = std::time::Instant::now();
        
        let personality = self.personality_core.read().await?;
        let mut orchestrator = self.system_orchestrator.write().await?;
        
        // Update content creation with error handling
        self.content_creator.update_strategy(&personality).await?;
        
        // Monitor scene generation performance
        let scene_result = self.scene_generator
            .adapt_to_content(self.content_creator.get_current_content())
            .await;
            
        if let Err(e) = scene_result {
            error!("Scene generation failed: {}", e);
            // Implement fallback behavior
        }
        
        // VRChat integration with graceful degradation
        if let Some(vrchat) = &self.vrchat_controller {
            if let Err(e) = vrchat.sync_with_personality(&personality).await {
                warn!("VRChat sync failed, continuing with limited functionality: {}", e);
            }
        }
        
        // Log performance metrics
        let duration = start.elapsed();
        if duration > std::time::Duration::from_millis(100) {
            warn!("System sync took longer than expected: {:?}", duration);
        }
        
        Ok(())
    }
}

impl Drop for SystemIntegrator {
    fn drop(&mut self) {
        // Ensure clean shutdown of all subsystems
        if let Some(vrchat) = &self.vrchat_controller {
            if let Err(e) = block_on(vrchat.shutdown()) {
                error!("VRChat shutdown error: {}", e);
            }
        }
        
        // Cleanup other resources
        if let Err(e) = block_on(self.scene_generator.cleanup()) {
            error!("Scene generator cleanup error: {}", e);
        }
    }
} 