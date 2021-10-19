use log::debug;
use redis::{AsyncCommands, Value};
use std::sync::Arc;

use errors::GnapError;
use model::transaction::TransactionOptions;

pub mod cache;
pub mod db;
mod constants;

use db::GnapDB;
use cache::GnapCache;
use constants::*;

#[derive(Clone)]
pub struct Service {
    pub db_client: GnapDB,
    pub cache_client: GnapCache,
}

impl Service {
    pub fn new(db_client: GnapDB, cache_client: GnapCache) -> Self {
        Self {
            db_client,
            cache_client,
        }
    }

    pub async fn create() -> Arc<Service> {
           // Create the db and cache instances.  This should really migrate to the
    // Service module.  But it works for now.
        let db_client = GnapDB::new().await;
        let cache_client = GnapCache::new().await;
        let svc = Service::new(db_client, cache_client);
        Arc::new(svc)
    }

    pub async fn get_dbs(&self) -> Vec<String> {
        self.db_client.list_databases().await.expect("bug!")
    }

    pub async fn get_grant_options(&self) -> Result<TransactionOptions, GnapError> {
        //let cache_key = format!("{}:{}", CACHE_KEY_PREFIX, CACHE_TX_OPTIONS);
        let cache_key = "gnap:tx_options";
        let mut con = self.cache_client.client.get_async_connection().await?;
        let cache_response = con.get(cache_key).await?;

        match cache_response {
            Value::Nil => {
                debug!("Use database to retrieve TransactionOptions");
                let result = self.db_client.fetch_grant_options().await?;
                let _: () = redis::pipe()
                    .atomic()
                    .set(CACHE_TX_OPTIONS, &result)
                    .expire(CACHE_TX_OPTIONS, 3600)
                    .query_async(&mut con)
                    .await?;

                Ok(result)
            }
            Value::Data(val) => {
                debug!("Use cache to retrieve TransactionOptions");
                Ok(serde_json::from_slice(&val)?)
            }
            _ => {
                debug!("Did not successfully get a cache response");
                Err(GnapError::GeneralError)
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
