use actix_web::{web};
use crate::{handlers};

// this function could be located in different module
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/.well-known")
            .service(web::resource("/openid-configuration").route(web::get().to(handlers::well_known::openid_config)))
            .service(web::resource("/gnap-as-rs").route(web::get().to(handlers::well_known::gnap_config)))
    );
}

