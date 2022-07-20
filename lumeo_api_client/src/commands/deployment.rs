use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Start deployment payload
#[derive(Debug, Serialize, Deserialize)]
pub struct StartDeployment {
    /// Deployment ID
    pub id: Uuid,
    /// Pipeline description
    pub pipeline: String,
}

/// Restart deployment payload
#[derive(Debug, Serialize, Deserialize)]
pub struct RestartDeployment {
    /// Deployment ID
    pub id: Uuid,
    /// Pipeline description
    pub pipeline: String,
}

/// Stop deployment payload
#[derive(Debug, Serialize, Deserialize)]
pub struct StopDeployment {
    /// Deployment ID
    pub id: Uuid,
}
