use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use super::Client;
use crate::Result;

#[skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Client {
    pub async fn get_orgs(&self) -> Result<Vec<Organization>> {
        self.get("/v1/orgs", Option::<&()>::None).await
    }
}
