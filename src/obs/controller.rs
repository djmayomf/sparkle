use obws::Client as OBSClient;
use crate::error::{AppError, Result};

pub struct OBSController {
    client: OBSClient,
}

impl OBSController {
    pub async fn new() -> Result<Self> {
        let client = OBSClient::connect("localhost", 4455)
            .await
            .map_err(|e| AppError::OBS(e.to_string()))?;
            
        Ok(Self { client })
    }
} 