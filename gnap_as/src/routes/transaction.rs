use crate::handlers;
use actix_web::{http, web};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/gnap").service(
            web::resource("/tx")
                .route(web::post().to(handlers::transaction::grant_request))
                .route(web::method(http::Method::OPTIONS).to(handlers::transaction::grant_options)),
        ),
    );
}