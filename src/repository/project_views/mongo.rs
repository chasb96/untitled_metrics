use crate::repository::{error::QueryError, mongo::MongoDatabase};
use mongodb::bson::doc;

use super::ProjectViewsRepository;

impl ProjectViewsRepository for MongoDatabase {
    async fn increment_view_count(&self, project_id: &str) -> Result<(), QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<()>("project_views")
            .update_one(
                doc! { "project_id": project_id },
                doc! { 
                    "$inc": { "views": 1 },
                    "upsert": true,
                },
            )
            .await
            .map(|_| ())
            .map_err(QueryError::from)
    }
}