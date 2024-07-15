use super::{error::QueryError, mongo::MongoDatabase};

mod mongo;

pub trait ProjectViewsRepository {
    async fn increment_view_count(&self, project_id: &str) -> Result<(), QueryError>;
}

#[allow(dead_code)]
pub enum ProjectViewsRepositoryOption {
    Mongo(MongoDatabase),
}

impl ProjectViewsRepository for ProjectViewsRepositoryOption {
    async fn increment_view_count(&self, project_id: &str) -> Result<(), QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.increment_view_count(project_id).await,
        }
    }
}

impl Default for ProjectViewsRepositoryOption {
    fn default() -> Self {
        Self::Mongo(Default::default())
    }
}