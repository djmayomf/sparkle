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
    pub async fn synchronize_systems(&mut self) -> Result<()> {
        // Ensure all systems are in sync
        let personality = self.personality_core.read().await;
        let mut orchestrator = self.system_orchestrator.write().await;
        
        // Update content creation based on personality state
        self.content_creator.update_strategy(&personality).await?;
        
        // Sync scene generation with content
        self.scene_generator.adapt_to_content(
            self.content_creator.get_current_content()
        ).await?;
        
        // If VRChat is enabled, sync its state
        if let Some(vrchat) = &self.vrchat_controller {
            vrchat.sync_with_personality(&personality).await?;
        }
        
        Ok(())
    }
} 