use crate::handlers;
use actix_web::{web};

/*
pub fn add_client() -> BoxedFilter<(GnapClientRequest, )> {
    trace!("add_client router");
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
    warp::put()
    .and(path_prefix())
    .and(path("client"))
//    .and(path::end())
    .and(json_body)
    .boxed()
}
*/

/*
pub fn update_client() -> BoxedFilter<(GnapClient, Uuid)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
    warp::post()
    .and(path_prefix())
    .and(path("client"))
    .and(warp::path::param())
    .and(json_body).boxed()
}
*/

// this function could be located in different module
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/db")
            .service(web::resource("/client/{id}").route(web::get().to(handlers::db::get_client)))
            .service(web::resource("/client").route(web::put().to(handlers::db::add_client))),
    );
}
