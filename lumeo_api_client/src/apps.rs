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
    pub async fn get_app(&self, application_id: &Uuid) -> Result<Application> {
        let path = format!("/v1/apps/{application_id}");
        self.get(&path, None::<&()>).await
    }

    pub async fn get_apps(&self, organization_id: &Uuid) -> Result<Vec<Application>> {
        let path = format!("/v1/orgs/{organization_id}/apps");
        self.get(&path, None::<&()>).await
    }
}
