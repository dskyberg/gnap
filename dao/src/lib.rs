use log::{trace, debug};
use redis::{AsyncCommands, Value};
use uuid::Uuid;
use errors::GnapError;
use model::{
    transaction::TransactionOptions,
    client::{GnapClient, GnapClientRequest}
};

use db::GnapDB;
use cache::GnapCache;
use constants::*;

pub mod cache;
pub mod db;
mod constants;

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

    pub async fn create() -> Service {
           // Create the db and cache instances.  This should really migrate to the
    // Service module.  But it works for now.
        let db_client = GnapDB::new().await;
        let cache_client = GnapCache::new().await;
        Service::new(db_client, cache_client)

    }

    pub async fn get_dbs(&self) -> Vec<String> {
        self.db_client.list_databases().await.expect("bug!")
    }

    pub async fn get_grant_options(&self) -> Result<TransactionOptions, GnapError> {
        //let cache_key = format!("{}:{}", CACHE_KEY_PREFIX, CACHE_TX_OPTIONS);
        let cache_key = "gnap:tx_options".to_owned();
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

    pub async fn add_client(&self, request: GnapClientRequest) -> Result<GnapClient, GnapError> {
        let client = self.db_client.add_client(request).await?;
        let mut con = self.cache_client.client.get_async_connection().await?;
        let cache_key = format!("gnap:clients:{}", client.client_id.to_string()).to_owned();
        let _: () = redis::pipe()
        .atomic()
        .set(&cache_key, &client.clone())
        .expire(&cache_key, 3600)
        .query_async(&mut con)
        .await?;

        Ok(client)
    }

    pub async fn get_client(&self, id: &Uuid) -> Result<Option<GnapClient>, GnapError> {
        trace!("Service - get_client");

        let cache_key = format!("gnap:clients:{}", id.to_string());
        let mut con = self.cache_client.client.get_async_connection().await?;
        let cache_response = con.get(&cache_key).await?;

        match cache_response {
            Value::Nil => {
                debug!("Use database to retrieve TransactionOptions");
                let result = self.db_client.fetch_client_by_id(&id).await?;
                if result.is_some() {
                    let data = result.unwrap();
                    let _: () = redis::pipe()
                    .atomic()
                    .set(CACHE_TX_OPTIONS, &data.clone())
                    .expire(CACHE_TX_OPTIONS, 3600)
                    .query_async(&mut con)
                    .await?;
                    Ok(Some(data))
                }
                else {
                    Ok(None)
                }
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
