use chrono::{DateTime, Utc};
use fn_error_context::context;
use lumeo_pipeline::Pipeline;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use super::Client;

#[skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct Deployment {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pipeline_id: Uuid,
    pub device_id: Uuid,
    pub state: State,
    #[serde(with = "crate::util::json_string")]
    pub definition: Pipeline,
}

#[derive(Debug, Deserialize, Serialize, strum::AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum State {
    Deploying,
    Running,
    Stopping,
    Stopped,
    Interrupted,
    Error,
    Unknown,
}

#[derive(Debug, Default, Serialize)]
pub struct ListParams {
    /// Maximum number of deployments to return
    pub limit: i16,
    /// Filter: Lower bound for creation time (inclusive)
    pub created_ts_since: Option<DateTime<Utc>>,
    /// Filter: Upper bound for creation time (exclusive)
    pub created_ts_until: Option<DateTime<Utc>>,
    /// Filter: Lower bound for update time (inclusive)
    pub updated_ts_since: Option<DateTime<Utc>>,
    /// Filter: Upper bound for update time (exclusive)
    pub updated_ts_until: Option<DateTime<Utc>>,
    /// Filter: Pipeline ID(s)
    pub pipeline_ids: Vec<Uuid>,
    /// Filter: Device ID(s)
    pub device_ids: Vec<Uuid>,
    /// Filter: State(s)
    pub states: Vec<State>,
}

impl Client {
    #[context("Getting deployments")]
    pub async fn get_deployments(&self, filter: &ListParams) -> anyhow::Result<Vec<Deployment>> {
        let path =
            format!("/v1/apps/{}/deployments?new_def_field_name=true", self.application_id()?);
        self.get(&path, Some(&filter)).await
    }

    #[context("Getting pipeline for deployment {}", id)]
    pub async fn get_deployment_definition(&self, id: Uuid) -> anyhow::Result<Pipeline> {
        let path = format!("/v1/apps/{}/deployments/{}/definition", self.application_id()?, id);
        self.get(&path, None::<&()>).await
    }
}
