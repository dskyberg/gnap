use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;
use uuid::Uuid;
use log::{trace, debug};
use errors::GnapError;
use model::transaction::TransactionOptions;
use model::client::{GnapClient, GnapClientRequest};


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

    // Figure out how to break these out into separate mods, so this file
    // is manageable.
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

    // Client methods
    pub async fn fetch_client_by_id(&self, id: &Uuid) -> Result<Option<GnapClient>, GnapError> {
        trace!("Fetching client by ID: {}", id.to_string());
        let cursor_result = self
            .database
            .collection::<GnapClient>("clients")
            .find_one(doc!{"client_id": &id.to_string()}, None)
            .await
            .map_err(GnapError::DatabaseError);
        match cursor_result {
            Ok(cursor) => {
                match cursor {
                    Some(result) => {
                        trace!("Fetched a client");
                        Ok(Some(result))
                    },
                    None => {
                        trace!("Client not found");
                        Err(GnapError::NotFound)
                    },
                }
            },
            Err(e) =>{
                trace!("get_client returned en error: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn add_client(&self, request: GnapClientRequest) -> Result<GnapClient, GnapError> {
        let collection = self
            .database
            .collection::<GnapClient>("clients");
        let client = GnapClient::new(request.redirect_uris, request.client_name );
        match collection.insert_one(client.clone(), None).await {
            Ok(_) => {
                debug!("Added client: {:?}", &client);
                Ok(client)
            },
            Err(err) => {
                debug!("Error saving client: {:?}", &err);
                Err(GnapError::DatabaseError(err))
            }
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
