use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;

use log::info;
use pretty_env_logger;

use gnap_as::{app_state, get_ip_addresses, tls_builder};
mod grant;
mod handlers;
mod routes;

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

    let (api_address, tls_address, ip) = get_ip_addresses();
    info!(
        "\nHTTP is running on {:?}\nHTTPS is running on {:?}\nIP address is {}",
        &api_address, &tls_address, &ip
    );

    // Set up the shared application state
    let app_state = app_state().await;

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
        .bind_openssl(tls_address, tls_builder())?
        .run()
        .await
}
