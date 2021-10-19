use warp::{filters::BoxedFilter, path, Filter};
use std::sync::Arc;
use crate::{handlers, routes::with_service::with_service};
use dao::Service;
use model::transaction::GrantRequest;

// Sets the path prefix for this API.
// host:port//<path prefix>/<specific API route>
fn path_prefix() -> BoxedFilter<()> {
    path!("gnap" / ..).boxed()
}

pub fn grant_post() -> BoxedFilter<(GrantRequest,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
    warp::post()
    .and(path_prefix())
    .and(path("tx"))
    .and(json_body).boxed()
}

pub fn grant_options() -> BoxedFilter<()> {
    warp::options() // 3.
        .and(path_prefix())
        .and(path("tx"))
        .boxed()
}

// Returns the collected set of routes
// The routes defined above are partners with middleware, such as services, and
// their respective route handlers.  The main function will combine these with
// any other routes defined in the service.
pub fn routes(services: &Arc<Service>) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    grant_options()
    .and(with_service(Arc::clone(services)))
    .and_then(handlers::transaction::grant_options)

    .or(
        grant_post()
        .and(with_service(Arc::clone(services)))
        .and_then(handlers::transaction::grant_post)
    )
}
