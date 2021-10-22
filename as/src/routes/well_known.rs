use warp::{filters::BoxedFilter, path, Filter};
use std::sync::Arc;
use crate::{handlers, routes::with_service::with_service};
use dao::Service;

// Sets the path prefix for this API.
// host:port//<path prefix>/<specific API route>
fn path_prefix() -> BoxedFilter<()> {
    path!(".well-known" / ..).boxed()
}

pub fn openid_config() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(path("openid-configuration"))
        .boxed()
}

pub fn gnap_config() -> BoxedFilter<()> {
    warp::get()
    .and(path_prefix())
    .and(path("gnap-as-rs"))
    .boxed()
}

pub fn routes(services: &Arc<Service>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {

    openid_config()
    .and(with_service(Arc::clone(services)))
    .and_then(handlers::well_known::openid_config)
    .or(
        gnap_config()
        .and(with_service(Arc::clone(services)))
        .and_then(handlers::well_known::gnap_config)
    )
}