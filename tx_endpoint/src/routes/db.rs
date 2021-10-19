use warp::{filters::BoxedFilter, path, Filter};
use std::sync::Arc;
use crate::{handlers, routes::with_service::with_service};
use dao::Service;

// Sets the path prefix for this API.
// host:port//<path prefix>/<specific API route>
fn path_prefix() -> BoxedFilter<()> {
    path!("db" / ..).boxed()
}


pub fn get_client() -> BoxedFilter<(String,)> {
    warp::get() // 3.
        .and(path_prefix())
        .and(path("client"))
        .and(warp::path::param())
        .boxed()
}

pub fn routes(services: &Arc<Service>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    get_client()
    .and(with_service(Arc::clone(services)))
    .and_then(handlers::db::get_client)
}
