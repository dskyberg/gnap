use actix_web::{web, HttpResponse};
use log::trace;
use dao::service::Service;
use model::{
    oidc::OpenIDConfiguration
};
use errors::GnapError;


/*
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

*/

pub async fn openid_config(
    _service: web::Data<Service>
) -> HttpResponse {
    trace!("openid_config");

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

    HttpResponse::Ok().json(config)
}

pub async fn gnap_config(
    service: web::Data<Service>
) -> HttpResponse {
    //let config = GnapOptions::new();
    let result = service.get_gnap_well_knowns().await;

    match result {
        Ok(response) => HttpResponse::Ok().json(&response),
        Err(err) => {
            match err {
                GnapError::BadData => HttpResponse::BadRequest().body("Missing GNAP options"),
                _ => HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }

}

