use chrono::{DateTime, Utc};
use fn_error_context::context;
use serde::Deserialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use super::Client;

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
    #[context("Getting organizations")]
    pub async fn get_orgs(&self) -> anyhow::Result<Vec<Organization>> {
        self.get("/v1/orgs", Option::<&()>::None).await
    }
}
