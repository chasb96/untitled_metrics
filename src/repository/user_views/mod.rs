use super::{error::QueryError, mongo::MongoDatabase};

mod mongo;

pub trait UserViewsRepository {
    async fn increment_view_count(&self, user_id: &str) -> Result<(), QueryError>;
}

#[allow(dead_code)]
pub enum UserViewsRepositoryOption {
    Mongo(MongoDatabase),
}

impl UserViewsRepository for UserViewsRepositoryOption {
    async fn increment_view_count(&self, user_id: &str) -> Result<(), QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.increment_view_count(user_id).await,
        }
    }
}

impl Default for UserViewsRepositoryOption {
    fn default() -> Self {
        Self::Mongo(Default::default())
    }
}