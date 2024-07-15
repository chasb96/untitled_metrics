use std::{error::Error, fmt::{self, Display}};

use deadpool::managed::PoolError;
use mongodb::error::Error as MongoError;

#[derive(Debug)]
pub enum QueryError {
    Mongo(MongoError),
    MongoPool(PoolError<MongoError>),
}

impl Error for QueryError { }

impl Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Mongo(e) => write!(f, "Error from Mongo: {}", e),
            Self::MongoPool(e) => write!(f, "Error from Mongo Pool: {}", e),
        }
    }
}

impl From<MongoError> for QueryError {
    fn from(value: MongoError) -> Self {
        Self::Mongo(value)
    }
}

impl From<PoolError<MongoError>> for QueryError {
    fn from(value: PoolError<MongoError>) -> Self {
        Self::MongoPool(value)
    }
}