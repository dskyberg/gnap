use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use warp::{self, Filter};

//use log::{error, info, warn, LevelFilter};

use log4rs;

use dao::Service;

mod handlers;
mod routes;

/// Crate main.
/// The main service needs to be async, in order to leverage async services.
#[tokio::main]
async fn main() {
    // Load the values from `.env` into the environment.  Then we can use
    // normal std::env methods to access.
    dotenv().ok();

    // Configure logging.  Update the log4rs.yml file to modify the config.
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let api_address: SocketAddr = env::var("API_ADDRESS")
        .expect("API_ADDRESS is not set in env")
        .parse()
        .expect("API_ADDRESS is invalid");

    let service = Service::create().await;

    // Generate the routes collection.  to extend, just add more `.or(macro)` calls.
    let routes = warp::path("app").and(warp::fs::dir("public"))
    .or(routes::db::routes(&service))
    .or(routes::well_known::routes(&service))
    .or(routes::transaction::routes(&service));

    // Start the service
    warp::serve(routes).run(api_address).await;
}



