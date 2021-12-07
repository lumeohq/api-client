use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use super::Client;
use crate::Result;

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
    pub async fn get_app(&self, app_id: &Uuid) -> Result<Application> {
        let path = format!("/v1/apps/{}", app_id);
        self.get(&path, Option::<&()>::None).await
    }

    pub async fn get_apps(&self, org_id: &Uuid) -> Result<Vec<Application>> {
        let path = format!("/v1/orgs/{}/apps", org_id);
        self.get(&path, Option::<&()>::None).await
    }
}
