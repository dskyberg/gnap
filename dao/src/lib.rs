use log::{trace, debug};
use redis::{AsyncCommands, Value};
use uuid::Uuid;
use errors::GnapError;
use model::{
    CachePath,
    transaction::{GnapTransaction, TransactionOptions},
    grant::GrantRequest,
    client::{GnapClient, GnapClientRequest},
    account::Account,
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
        let cache_key = TransactionOptions::cache_path();
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
        let cache_key = format!("{}:{}",GnapClient::cache_path(), client.client_id.to_string()).to_owned();
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

        let cache_key = format!("{}:{}", GnapClient::cache_path(), id.to_string());
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

    pub async fn get_account(&self, id: &Uuid) -> Result<Option<Account>, GnapError> {
        trace!("Service - get_account");

        let cache_key = format!("{}:{}", Account::cache_path(), id.to_string());
        let mut con = self.cache_client.client.get_async_connection().await?;
        let cache_response = con.get(&cache_key).await?;

        match cache_response {
            Value::Nil => {
                trace!("Use database to retrieve Account");
                let result = self.db_client.fetch_account_by_id(&id).await?;
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
                trace!("Use cache to retrieve Account");
                Ok(serde_json::from_slice(&val)?)
            }
            _ => {
                debug!("Did not successfully get a cache response");
                Err(GnapError::GeneralError)
            }
        }
    }

    /// Start a GNAP transaction.
    /// This is called from the grant request handler.  The request is cached
    /// with the transaction. Ownership of the request passes to the transaction.
    pub async fn start_transaction(&self, request: GrantRequest) -> Result<GnapTransaction, GnapError> {
        let mut con = self.cache_client.client.get_async_connection().await?;
        let tx = GnapTransaction::new(Some(request));
        let cache_key = format!("{}:{}",GnapTransaction::cache_path(), &tx.tx_id.clone());
        let _: () = redis::pipe()
        .atomic()
        .set(&cache_key, &tx.clone())
        .expire(&cache_key, 3600)
        .query_async(&mut con)
        .await?;
        Ok(tx)
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
