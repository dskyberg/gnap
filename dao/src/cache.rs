use redis::aio::ConnectionManager;
use redis::Client;
use std::env;

#[derive(Clone)]
pub struct GnapCache {
    pub client: Client,
    pub connection_manager: ConnectionManager,
}

impl GnapCache {
    pub async fn new() -> Self {
        let redis_uri = env::var("REDIS_URI").expect("REDIS_URI env var should be specified");

        let client = Client::open(redis_uri).expect("Failed to open Redis client");

        let connection_manager = client
            .get_tokio_connection_manager()
            .await
            .expect("Can't create Redis connection manager");

        Self {
            client,
            connection_manager,
        }
    }
}


