use deadpool::managed::{Manager, Metrics, RecycleError, RecycleResult};
use mongodb::bson::doc;

pub struct MongoConnectionManager {
    pub client: mongodb::Client,
    pub database: String,
}

impl Manager for MongoConnectionManager {
    type Type = mongodb::Database;
    type Error = mongodb::error::Error;
    
    async fn create(&self) -> Result<mongodb::Database, Self::Error> {
        Ok(self.client.database(&self.database))
    }
    
    async fn recycle(&self, conn: &mut mongodb::Database, _: &Metrics) -> RecycleResult<Self::Error> {
        conn.run_command(doc! { "ping": 1 })
            .await
            .map(|_| ())
            .map_err(|_| RecycleError::message("Failed to ping mongodb"))
    }
}