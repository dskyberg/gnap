use warp;
use std::sync::Arc;
use dao::Service;
use model::transaction::{GrantRequest, TransactionOptions};

pub async fn grant_options(service: Arc<Service>) -> Result<impl warp::Reply, warp::Rejection> {

    //let options = GrantOptions::new();
    let options: TransactionOptions = service.get_grant_options().await?;
    Ok(warp::reply::json(&options))
}

pub async fn grant_post(request: GrantRequest, _service: Arc<Service>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&request))
}