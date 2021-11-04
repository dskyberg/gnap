use model::grant::*;
use dotenv::dotenv;
use pretty_env_logger;
use std::error::Error as StdError;
use gnap_client::make_request;
use model::gnap::GnapOptions;
use log::trace;

const GNAP_AS_HOST: &str = "http://localhost:8000";

fn as_path(part: &str) -> String {
    format!("{}/{}", GNAP_AS_HOST, part)
}

async fn get_config() -> Result<GnapOptions, Box<dyn StdError>> {

    let path = as_path(".well-known/gnap-as-rs");
    trace!("Using path: {}", &path);
    let response: GnapOptions = reqwest::Client::new()
        .get(&path)
        .send()
        .await?
        .json()
        .await?;
    println!("{:?}", &response);
    Ok(response)
}

/// Using the tokio runtime via actix_web.
/// When we extend the client with a user agent (browser), it will be an easy
/// extension.
#[actix_web::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    dotenv().ok();
    pretty_env_logger::init();

    // Get the GNAP well knowns from the server
    let options = get_config().await?;

    let request = make_request();
    println!("{:?}", &request);
    trace!("Using {}", &options.service_endpoints.grant_request_endpoint);
    let response: GrantResponse = reqwest::Client::new()
        .post(options.service_endpoints.grant_request_endpoint)
        .json(&request)
        .send()
        .await?
        .json()
        .await?;

    // server http response
    println!("Response: {:?}", response);

    Ok(())
}
