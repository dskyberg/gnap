use warp;
use std::sync::Arc;
use dao::Service;
use model::client::GnapClient;

pub async fn get_client(id: String, _service: Arc<Service>,) -> Result<impl warp::Reply, warp::Rejection> {

    let redirect_uris = vec!["https://client.example.com".to_owned()];
    let client_name = id;
    let client = GnapClient::new(redirect_uris, client_name);
    Ok(warp::reply::json(&client))
}

