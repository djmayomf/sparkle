use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::PathBuf;
use windows_automation::{ElementHandle, MouseButton};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelingSoftwareController {
    pub photoshop: PhotoshopAutomation,
    pub live2d: Live2DCubismAutomation,
    pub vtube_studio: VTubeStudioAutomation,
    pub working_directory: PathBuf,
    pub current_task: Option<ModelingTask>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoshopAutomation {
    pub process_handle: Option<u32>,
    pub window_handle: Option<ElementHandle>,
    pub current_document: Option<String>,
    pub active_layer: Option<String>,
    pub scripts_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Live2DCubismAutomation {
    pub process_handle: Option<u32>,
    pub window_handle: Option<ElementHandle>,
    pub current_model: Option<String>,
    pub current_parameter: Option<String>,
    pub deformer_active: bool,
}

impl ModelingSoftwareController {
    pub async fn new(working_dir: PathBuf) -> Self {
        Self {
            photoshop: PhotoshopAutomation::new(),
            live2d: Live2DCubismAutomation::new(),
            vtube_studio: VTubeStudioAutomation::new(),
            working_directory: working_dir,
            current_task: None,
        }
    }

    pub async fn start_modeling_session(&mut self) -> Result<(), String> {
        // Launch required software
        self.launch_photoshop().await?;
        self.launch_live2d().await?;
        
        // Load project files
        self.load_project_files().await?;
        
        Ok(())
    }

    async fn launch_photoshop(&mut self) -> Result<(), String> {
        let process = Command::new("C:\\Program Files\\Adobe\\Adobe Photoshop 2024\\Photoshop.exe")
            .arg("--no-splash")
            .spawn()
            .map_err(|e| e.to_string())?;

        self.photoshop.process_handle = Some(process.id());
        self.wait_for_window("Photoshop").await?;
        
        // Initialize workspace
        self.photoshop.load_workspace("Live2D_Modeling").await?;
        self.photoshop.run_script("initialize_live2d_layers.jsx").await?;
        
        Ok(())
    }

    async fn launch_live2d(&mut self) -> Result<(), String> {
        let process = Command::new("C:\\Program Files\\Live2D Cubism 4.1\\CubismEditor.exe")
            .spawn()
            .map_err(|e| e.to_string())?;

        self.live2d.process_handle = Some(process.id());
        self.wait_for_window("Live2D Cubism Editor").await?;
        
        // Initialize workspace
        self.live2d.load_workspace("ModelCreation").await?;
        
        Ok(())
    }

    pub async fn create_model_part(&mut self, part: ModelPart) -> Result<(), String> {
        match part {
            ModelPart::Face => self.create_face().await?,
            ModelPart::Hair => self.create_hair().await?,
            ModelPart::Body => self.create_body().await?,
            ModelPart::Accessories => self.create_accessories().await?,
        }
        Ok(())
    }

    async fn create_face(&mut self) -> Result<(), String> {
        // Switch to Photoshop
        self.photoshop.activate_window().await?;
        
        // Create face structure
        self.photoshop.create_layer_group("Face").await?;
        self.photoshop.create_layers(vec![
            "Base",
            "Shadows",
            "Highlights",
            "Details",
        ]).await?;
        
        // Export for Live2D
        self.photoshop.export_for_live2d("face").await?;
        
        // Switch to Live2D
        self.live2d.activate_window().await?;
        
        // Set up face parameters
        self.live2d.create_parameter_group("Face").await?;
        self.live2d.add_deformers(vec![
            "EyeL", "EyeR",
            "MouthForm", "MouthOpen",
            "Eyebrow_L", "Eyebrow_R"
        ]).await?;
        
        Ok(())
    }

    pub async fn apply_physics(&mut self, component: PhysicsComponent) -> Result<(), String> {
        self.live2d.activate_window().await?;
        
        match component {
            PhysicsComponent::Hair => {
                self.live2d.add_physics_group("Hair").await?;
                self.live2d.set_physics_parameters(vec![
                    ("Gravity", 1.0),
                    ("Wind", 0.3),
                    ("Bounce", 0.5),
                ]).await?;
            },
            PhysicsComponent::Clothes => {
                self.live2d.add_physics_group("Clothes").await?;
                self.live2d.set_physics_parameters(vec![
                    ("Gravity", 0.8),
                    ("Wind", 0.2),
                    ("Stiffness", 0.7),
                ]).await?;
            },
            // Add other physics components
        }
        
        Ok(())
    }

    pub async fn test_model(&mut self) -> Result<TestResults, String> {
        // Switch to VTube Studio
        self.vtube_studio.activate_window().await?;
        
        // Run movement tests
        let movement_results = self.vtube_studio.test_movements().await?;
        
        // Test expressions
        let expression_results = self.vtube_studio.test_expressions().await?;
        
        // Test physics
        let physics_results = self.vtube_studio.test_physics().await?;
        
        Ok(TestResults {
            movement_score: movement_results.score,
            expression_score: expression_results.score,
            physics_score: physics_results.score,
            issues: vec![],
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModelPart {
    Face,
    Hair,
    Body,
    Accessories,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PhysicsComponent {
    Hair,
    Clothes,
    Accessories,
    TailMovement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResults {
    pub movement_score: f32,
    pub expression_score: f32,
    pub physics_score: f32,
    pub issues: Vec<String>,
} 