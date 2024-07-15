use crate::repository::{error::QueryError, mongo::MongoDatabase};
use mongodb::bson::doc;

use super::UserViewsRepository;

impl UserViewsRepository for MongoDatabase {
    async fn increment_view_count(&self, user_id: &str) -> Result<(), QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<()>("user_views")
            .update_one(
                doc! { "user_id": user_id },
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