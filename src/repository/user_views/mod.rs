use super::{error::QueryError, mongo::MongoDatabase, MetricUser};

mod mongo;

pub trait UserViewsRepository {
    async fn increment_view_count(&self, user_id: &str) -> Result<(), QueryError>;

    async fn popular(&self) -> Result<Vec<MetricUser>, QueryError>;
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

    async fn popular(&self) -> Result<Vec<MetricUser>, QueryError> {
        match self {
            Self::Mongo(mongo) => mongo.popular().await,
        }
    }
}

impl Default for UserViewsRepositoryOption {
    fn default() -> Self {
        Self::Mongo(Default::default())
    }
}