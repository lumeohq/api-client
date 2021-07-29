use fn_error_context::context;
use uuid::Uuid;

use super::Client;

impl Client {
    #[context("Deleting file {}", id)]
    pub async fn delete_file(&self, id: Uuid) -> anyhow::Result<()> {
        self.delete(&format!("/v1/apps/{}/files/{}", self.application_id()?, id)).await
    }
}
