use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub enum AppType {
    VTubeStudio,
    VRChat,
    VMagicMirror,
    PrprLive,
    Custom(String),
}

#[async_trait]
pub trait AppAdapter {
    async fn initialize(&mut self) -> Result<()>;
    async fn get_tracking_data(&self) -> Result<TrackingData>;
    async fn update_model_state(&self, state: &ModelState) -> Result<()>;
    async fn adapt_animation(&self, animation: Animation) -> Result<Animation>;
    async fn play_animation(&self, animation: &Animation) -> Result<()>;
    async fn adapt_expression(&self, expression: Expression) -> Result<Expression>;
    async fn adapt_movement(&self, movement: Movement) -> Result<Movement>;
    async fn map_emotion_to_expressions(&self, emotion: Emotion) -> Result<Vec<Expression>>;
    async fn map_emotion_to_gestures(&self, emotion: Emotion) -> Result<Vec<Gesture>>;
}

pub struct AppInterface {
    app_type: AppType,
    adapter: Box<dyn AppAdapter>,
    state: AppState,
}

impl AppInterface {
    pub fn new(app_type: AppType) -> Self {
        let adapter: Box<dyn AppAdapter> = match app_type {
            AppType::VTubeStudio => Box::new(VTubeStudioAdapter::new()),
            AppType::VRChat => Box::new(VRChatAdapter::new()),
            AppType::VMagicMirror => Box::new(VMagicMirrorAdapter::new()),
            AppType::PrprLive => Box::new(PrprLiveAdapter::new()),
            AppType::Custom(name) => Box::new(CustomAdapter::new(name)),
        };

        Self {
            app_type,
            adapter,
            state: AppState::default(),
        }
    }

    // Delegate methods to the appropriate adapter
    pub async fn get_tracking_data(&self) -> Result<TrackingData> {
        self.adapter.get_tracking_data().await
    }

    pub async fn update_model_state(&self, state: &ModelState) -> Result<()> {
        self.adapter.update_model_state(state).await
    }

    // ... implement other adapter methods ...
} 