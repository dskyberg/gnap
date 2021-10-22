use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum GnapError {
    #[error("mongodb error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] mongodb::bson::document::ValueAccessError),
    #[error("Cache error: {0}")]
    CacheError(#[from] redis::RedisError),
    #[error("Not found error")]
    NotFound,
    #[error("General error")]
    GeneralError
}

impl From<serde_json::Error> for GnapError {
    fn from(_source: serde_json::Error) -> Self {
        Self::GeneralError
    }
}

impl warp::reject::Reject for GnapError {}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
