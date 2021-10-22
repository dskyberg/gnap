use std::sync::Arc;
use warp::{self, Filter};
use dao::Service;

pub fn with_service(
    service: Arc<Service>,
) -> impl Filter<Extract = (Arc<Service>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&service))
}
