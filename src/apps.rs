use chrono::{DateTime, Utc};
use fn_error_context::context;
use serde::Deserialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use super::Client;

#[skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct Application {
    pub id: Uuid,
    pub name: String,
    pub organization_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Client {
    #[context("Getting application")]
    pub async fn get_app(&self, app_id: &Uuid) -> anyhow::Result<Application> {
        let path = format!("/v1/apps/{}", app_id);
        self.get(&path, Option::<&()>::None).await
    }

    #[context("Getting applications")]
    pub async fn get_apps(&self, org_id: &Uuid) -> anyhow::Result<Vec<Application>> {
        let path = format!("/v1/orgs/{}/apps", org_id);
        self.get(&path, Option::<&()>::None).await
    }
}
