use chrono::{DateTime, Utc};
use fn_error_context::context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Client;

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

impl Client {
    #[context("Creating event")]
    pub async fn create_event(&self, event: &EventData) -> anyhow::Result<Event> {
        self.post(&format!("/v1/apps/{}/events", self.application_id()?), event).await
    }
}
