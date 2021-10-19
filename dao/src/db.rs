use futures::stream::TryStreamExt;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;

use errors::GnapError;
use model::transaction::TransactionOptions;

#[derive(Clone, Debug)]
pub struct GnapDB {
    pub client: Client,
    pub database: Database
}

//const MONGO_URI: &str = "mongodb://127.0.0.1:27017";

impl GnapDB {
    pub async fn new() -> Self {
        // Read the config from either the environment or a .env file.
        let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI missing");
        let database = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE missing");
        let app_name = env::var("MONGODB_APP_NAME").expect("MONGODB_APP_NAME missing");

        // Create the ClientOptions and set the app_name
        let mut client_options = ClientOptions::parse(mongo_uri)
            .await
            .expect("Failed to create client options");
        client_options.app_name = Some(app_name);

        // Create the client and grab a database handle
        let client = Client::with_options(client_options).expect("Failed to create MongoDB client");
        let db = client.database(&database);
        Self {
            client: client,
            database: db
        }
    }

    pub async fn list_databases(&self) -> Result<Vec<String>, GnapError> {
        match self.client.list_database_names(None, None).await {
            Ok(v) => Ok(v),
            Err(e) => Err(GnapError::DatabaseError(e)),
        }
    }

    pub async fn fetch_grant_options(&self) -> Result<TransactionOptions, GnapError> {
        let mut cursor = self
            .database
            .collection::<TransactionOptions>("transaction_options")
            .find(None, None)
            .await
            .map_err(GnapError::DatabaseError)?;

        match cursor.try_next().await {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Ok(TransactionOptions::new()),
            Err(e) => Err(GnapError::DatabaseError(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
