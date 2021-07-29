use std::convert::TryFrom;

use anyhow::bail;
use chrono::{DateTime, Utc};
use fn_error_context::context;
use lumeo_events::deployment::{DeploymentEventKind, Event as DeploymentEvent};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Client;

#[derive(Deserialize, Serialize)]
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

impl TryFrom<&DeploymentEvent> for EventData {
    type Error = anyhow::Error;

    #[context("Converting DeploymentEvent to EventData")]
    fn try_from(deployment_event: &DeploymentEvent) -> Result<Self, Self::Error> {
        let event_type = match &deployment_event.event {
            DeploymentEventKind::Started => "deployment.started".to_string(),
            DeploymentEventKind::StartFailed => "deployment.start_failed".to_string(),
            DeploymentEventKind::Stopped => "deployment.stopped".to_string(),
            DeploymentEventKind::StopFailed => "deployment.stop_failed".to_string(),
            DeploymentEventKind::ExitedUnexpectedly => "deployment.exited_unexpectedly".to_string(),
            DeploymentEventKind::GstError(_) => "deployment.error.pipeline".to_string(),
            DeploymentEventKind::FunctionNodeError(_) => "deployment.error.function".to_string(),
            DeploymentEventKind::NodeLog(_) => {
                bail!("Node logs should not be handled");
            }
        };

        let severity = match deployment_event.event {
            DeploymentEventKind::Started => Severity::Info,
            DeploymentEventKind::StartFailed => Severity::Error,
            DeploymentEventKind::Stopped => Severity::Info,
            DeploymentEventKind::StopFailed => Severity::Warning,
            DeploymentEventKind::ExitedUnexpectedly => Severity::Error,
            DeploymentEventKind::GstError(_) => Severity::Error,
            DeploymentEventKind::FunctionNodeError(_) => Severity::Error,
            DeploymentEventKind::NodeLog(_) => {
                bail!("Node logs should not be handled");
            }
        };

        let description = match deployment_event.event {
            DeploymentEventKind::Started => "Deployment started".to_string(),
            DeploymentEventKind::StartFailed => "Deployment start failed".to_string(),
            DeploymentEventKind::Stopped => "Deployment stopped".to_string(),
            DeploymentEventKind::StopFailed => "Deployment stop failed".to_string(),
            DeploymentEventKind::ExitedUnexpectedly => "Deployment exited unexpectedly".to_string(),
            DeploymentEventKind::GstError(_) => {
                "Deployment encountered an unknown error".to_string()
            }
            DeploymentEventKind::FunctionNodeError(ref error) => {
                format!(
                    "Deployment encountered an error in Function node '{}':\n{}",
                    error.node_id, error.traceback
                )
            }
            DeploymentEventKind::NodeLog(_) => {
                bail!("Node logs should not be handled");
            }
        };

        Ok(Self {
            category: "deployment".to_string(),
            event_type,
            severity,
            payload: Some(description),
            object: Some("deployment".to_string()),
            object_id: Some(deployment_event.deployment_id),
        })
    }
}
