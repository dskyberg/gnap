use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use actix_web::{web, middleware, App, HttpServer};
use pretty_env_logger;
use log::info;

use dao::Service;

mod handlers;
mod routes;
mod utils;
mod grant;


/// Crate main.
/// The main service needs to be async, in order to leverage async services.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the values from `.env` into the environment.  Then we can use
    // normal std::env methods to access.
    dotenv().ok();

    // Configure logging.  Log defaults are set in RUST_LOG env.
    // Note:: bin namees in workspaces are strange.  Rather than `as`, this
    // binary is call `r#as`.
    pretty_env_logger::init();

    // This doesn't really havea ny value.  But fun to play with. We chould just
    // as easily pass the string from env::var into the HttpServer.bind func.
    let api_address: SocketAddr = env::var("API_ADDRESS")
        .expect("API_ADDRESS is not set in env")
        .parse()
        .expect("API_ADDRESS is invalid");

    // Get the local IP address of the non-loopback interface. This is just for
    // displaying at startup.
    let ip = utils::get_machine_ip();
    info!("Server is running on {:?}.  IP address is {}", api_address, ip);

    // Init the database and cache services
    let dao_service = Service::create().await;

    // App::app_data will wrap the app state in an Arc, so it is sharable
    let app_state =  web::Data::new(dao_service);

    // Create the actix-web App instance, with middleware and routes.
    let app = move || {
        App::new()
            // Enable app state data, including DB and Cache stuff.
            .app_data(app_state.clone())

            // Add each of the router modules.
            .configure(routes::db::routes)
            .configure(routes::well_known::routes)
            .configure(routes::transaction::routes)

            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
    };

    // Start http server with the app
    HttpServer::new(app)
    .bind(api_address)?
    .run()
    .await
}



