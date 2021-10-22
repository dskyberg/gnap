use warp;
use std::sync::Arc;
use dao::Service;
use model::{
    oidc::OpenIDConfiguration,
    gnap::GnapOptions
};

pub async fn openid_config(_service: Arc<Service>) -> Result<impl warp::Reply, warp::Rejection> {

    let issuer = "http://localhost:8000".to_owned();
    let authorization_endpoint = "http://localhost:8000/gnap/auth".to_owned();
    let token_endpoint = "http://localhost:8000/gnap/token".to_owned();
    let userinfo_endpoint = "http://localhost:8000/gnap/userinfo".to_owned();
    let jwks_uri = "http://localhost:8000/gnap/jwks".to_owned();

    let config: OpenIDConfiguration = OpenIDConfiguration::new(issuer,
        authorization_endpoint,
        token_endpoint,
        userinfo_endpoint,
        jwks_uri
    );
    Ok(warp::reply::json(&config))
}

pub async fn gnap_config(_service: Arc<Service>) -> Result<impl warp::Reply, warp::Rejection> {
    let config = GnapOptions::new();
    Ok(warp::reply::json(&config))
}