use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Client;
use crate::Result;

#[derive(Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(Serialize)]
pub struct EventData {
    pub category: String,
    pub event_type: String,
    pub severity: Severity,
    pub payload: Option<String>,
    pub object: Option<String>,
    pub object_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub event_ts: DateTime<Utc>,
    pub application_id: Uuid,
    pub category: String,
    pub event_type: String,
    pub severity: Severity,
    pub payload: Option<String>,
    pub object: Option<String>,
    pub object_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GstErrorDomain {
    Core = 1,
    Library = 2,
    Resource = 3,
    Stream = 4,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ErrorData {
    Deployment { deployment_id: Uuid, error: DeploymentError },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DeploymentError {
    GstError { domain: GstErrorDomain, code: i32 },
}

impl Client {
    pub async fn create_event(&self, event: &EventData) -> Result<Event> {
        let application_id = self.application_id()?;
        self.post(&format!("/v1/apps/{application_id}/events"), event).await
    }

    pub async fn create_error_event(&self, error_data: &ErrorData) -> Result<Event> {
        let application_id = self.application_id()?;
        self.post(&format!("/v1/internal/apps/{application_id}/events/error_events"), error_data)
            .await
    }
}
