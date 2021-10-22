use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use warp::{self, Filter};
use get_if_addrs;
//use log::{error, info, warn, LevelFilter};

use log4rs;

use dao::Service;

mod handlers;
mod routes;

fn get_ip() -> String {
    let addrs = get_if_addrs::get_if_addrs().unwrap();
   let ips = addrs.into_iter()
    .filter(|n| n.name != "lo0")
    .collect::<Vec<_>>();

    format!(" {:?}", ips[0].addr.ip())
}

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

    let ip = get_ip();
    println!("Server is running on {:?}.  IP address is {}", api_address, ip);
    let service = Service::create().await;

    // Generate the routes collection.  to extend, just add more `.or(macro)` calls.
    let routes = warp::path("app").and(warp::fs::dir("public"))
    .or(routes::db::routes(&service))
    .or(routes::well_known::routes(&service))
    .or(routes::transaction::routes(&service));

    // Start the service
    warp::serve(routes).run(api_address).await;
}



