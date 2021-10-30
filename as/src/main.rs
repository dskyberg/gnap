use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use actix_web::{web, middleware, App, HttpServer};

use get_if_addrs;
use log::info;
use pretty_env_logger;
//use log4rs;

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
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the values from `.env` into the environment.  Then we can use
    // normal std::env methods to access.
    dotenv().ok();

    // Configure logging.  Update the log4rs.yml file to modify the config.
    /*
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    */
    pretty_env_logger::init();
    let api_address: SocketAddr = env::var("API_ADDRESS")
        .expect("API_ADDRESS is not set in env")
        .parse()
        .expect("API_ADDRESS is invalid");

    let ip = get_ip();
    info!("Server is running on {:?}.  IP address is {}", api_address, ip);
    let service = Service::create().await;


    let addr = env::var("API_ADDRESS")
    .expect("API_ADDRESS is not set in env");

    let app = move || {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(service.clone()))
            .configure(routes::db::routes)
            .configure(routes::well_known::routes)
            .configure(routes::transaction::routes)
            //.service(web::resource("/auth").route(web::post().to(handlers::authenticate)))
    };

    // Start http server
    HttpServer::new(app)
    .bind(addr)?
    .run()
    .await
}



