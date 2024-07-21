use crate::repository::{error::QueryError, mongo::MongoDatabase, MetricProject};
use futures::TryStreamExt;
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

    async fn popular(&self) -> Result<Vec<MetricProject>, QueryError> {
        self.connection_pool
            .get()
            .await?
            .collection::<MetricProject>("project_views")
            .find(doc! { })
            .sort(doc! { "views": -1 })
            .limit(32)
            .await?
            .try_collect()
            .await
            .map_err(QueryError::from)
    }
}