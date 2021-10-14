use std::{collections::BTreeMap, fmt};

use chrono::{DateTime, Utc};
use fn_error_context::context;
use lumeo_pipeline::Pipeline;
use reqwest::Method;
use serde::{
    de::{self, value::SeqAccessDeserializer, Deserializer, Visitor},
    Deserialize, Serialize,
};
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
    #[serde(alias = "device_id")]
    pub gateway_id: Uuid,
    pub state: State,
    #[serde(deserialize_with = "deserialize_pipeline_def")]
    pub definition: Pipeline,
}

#[derive(Serialize)]
pub struct NewDeployment {
    pub pipeline_id: Uuid,
    #[serde(rename = "device_id")]
    pub gateway_id: Uuid,
    #[serde(flatten)]
    pub data: DeploymentData,
}

#[derive(Serialize)]
pub struct DeploymentData {
    pub name: Option<String>,
    pub state: Option<State>,
    pub definition: Option<String>,
    pub configuration: Option<DeploymentConfiguration>,
}

pub type DeploymentConfiguration = BTreeMap<String, serde_json::Map<String, serde_json::Value>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize, strum::AsRefStr)]
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
    #[serde(rename = "device_ids")]
    pub gateway_ids: Vec<Uuid>,
    /// Filter: State(s)
    pub states: Vec<State>,
}

impl Client {
    #[context("Getting deployments")]
    pub async fn get_deployments(&self, filter: &ListParams) -> anyhow::Result<Vec<Deployment>> {
        let path = format!("/v1/apps/{}/deployments", self.application_id()?);
        Ok(self.get(&path, Some(&filter)).await?)
    }

    #[context("Creating deployment")]
    pub async fn create_deployment(&self, data: &NewDeployment) -> anyhow::Result<Deployment> {
        let path = format!("/v1/apps/{}/deployments", self.application_id()?);
        Ok(self.post(&path, data).await?)
    }

    // FIXME: Make method naming consistent for all methods. It is either create/read/update/delete
    //        or post/get/put/delete.
    #[context("Getting deployment {}", id)]
    pub async fn get_deployment(&self, id: Uuid) -> anyhow::Result<Deployment> {
        let path = format!("/v1/apps/{}/deployments/{}", self.application_id()?, id);
        Ok(self.get(&path, None::<&()>).await?)
    }

    #[context("Updating deployment {}", id)]
    pub async fn update_deployment(
        &self,
        id: Uuid,
        data: &DeploymentData,
    ) -> anyhow::Result<Deployment> {
        let path = format!("/v1/apps/{}/deployments/{}", self.application_id()?, id);
        Ok(self.put(&path, data).await?)
    }

    #[context("Deleting deployment {}", id)]
    pub async fn delete_deployment(&self, id: Uuid) -> anyhow::Result<()> {
        let path = format!("/v1/apps/{}/deployments/{}", self.application_id()?, id);
        Ok(self.delete(&path).await?)
    }

    #[context("Getting pipeline for deployment {}", id)]
    pub async fn get_deployment_definition(&self, id: Uuid) -> anyhow::Result<Pipeline> {
        let path = format!("/v1/apps/{}/deployments/{}/definition", self.application_id()?, id);
        Ok(self.get(&path, None::<&()>).await?)
    }

    #[context("Starting deployment {}", id)]
    pub async fn start_deployment(&self, id: Uuid) -> anyhow::Result<()> {
        let path = format!("/v1/apps/{}/deployments/{}/start", self.application_id()?, id);
        self.request(Method::POST, &path, None)?.send().await?.error_for_status()?;
        Ok(())
    }

    #[context("Stopping deployment {}", id)]
    pub async fn stop_deployment(&self, id: Uuid) -> anyhow::Result<()> {
        let path = format!("/v1/apps/{}/deployments/{}/stop", self.application_id()?, id);
        self.request(Method::POST, &path, None)?.send().await?.error_for_status()?;
        Ok(())
    }
}

fn deserialize_pipeline_def<'de, D>(deserializer: D) -> Result<Pipeline, D::Error>
where
    D: Deserializer<'de>,
{
    struct PipelineVisitor;

    impl<'de> Visitor<'de> for PipelineVisitor {
        type Value = Pipeline;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a possibly-stringified pipeline definition")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            serde_json::from_str(v).map_err(de::Error::custom)
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            Pipeline::deserialize(SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(PipelineVisitor)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use lumeo_pipeline::Pipeline;
    use serde::Serialize;
    use serde_json::json;
    use uuid::Uuid;

    use super::{Deployment, State};

    #[test]
    fn deserialize_deployment() {
        let pipeline_json = json!([{
            "id": "video1",
            "properties": {
                "type": "video",
                "source_type": "stream",
                "source_id": Uuid::nil(),
                "resolution": "1280x720",
                "rtsp": { "name": "Video Name", "uri": "https://example.com/somevideo.mp4" },
            },
            "wires": {},
        }]);
        let pipeline_json_s = serde_json::to_string(&pipeline_json).unwrap();

        let pipeline: Pipeline = serde_json::from_value(pipeline_json.clone()).unwrap();

        let timestamp = Utc::now();

        let deployment: Deployment = serde_json::from_value(json!({
            "id": Uuid::nil(),
            "created_at": timestamp,
            "updated_at": timestamp,
            "pipeline_id": Uuid::nil(),
            "device_id": Uuid::nil(),
            "state": State::Stopped,
            // "Inlined" pipeline definition (array, not string)
            "definition": pipeline_json,
        }))
        .unwrap();
        assert_eq_json(&deployment.definition, &pipeline);

        let deployment: Deployment = serde_json::from_value(json!({
            "id": Uuid::nil(),
            "created_at": timestamp,
            "updated_at": timestamp,
            "pipeline_id": Uuid::nil(),
            "device_id": Uuid::nil(),
            "state": State::Stopped,
            // Stringified pipeline definition
            "definition": pipeline_json_s,
        }))
        .unwrap();
        assert_eq_json(&deployment.definition, &pipeline);
    }

    fn assert_eq_json<T: Serialize>(a: &T, b: &T) {
        let a_json = serde_json::to_string(a).unwrap();
        let b_json = serde_json::to_string(b).unwrap();

        assert_eq!(a_json, b_json);
    }
}
