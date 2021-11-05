use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::env;
use std::net::SocketAddr;

mod utils;

/// Get addresses from ENV
///
/// This doesn't really havea ny value.  But fun to play with. We could just
/// as easily pass the string from env::var into the HttpServer.bind func.

pub fn get_ip_addresses() -> (SocketAddr, SocketAddr, String) {
    let api_address: SocketAddr = env::var("API_ADDRESS")
        .expect("API_ADDRESS is not set in env")
        .parse()
        .expect("API_ADDRESS is invalid");

    let tls_address: SocketAddr = env::var("TLS_ADDRESS")
        .expect("TLS_ADDRESS is not set in env")
        .parse()
        .expect("TLS_ADDRESS is invalid");

    // Get the local IP address of the non-loopback interface. This is just for
    // displaying at startup.
    let ip = utils::get_machine_ip();

    (api_address, tls_address, ip)
}

/*

To create a self-signed temporary cert for testing, copy&paste the following:

    openssl req -x509 \
    -newkey rsa:4096 \
    -keyout .keystore/key.pem \
    -out .keystore/cert.pem \
    -sha256 \
    -days 3650 \
    -noenc \
    -subj '/CN=localhost' \
    -addext "basicConstraints = critical, CA:true" \
    -addext "keyUsage = critical, Digital Signature, Certificate Sign" \
    -addext "subjectKeyIdentifier=hash"

*/

/// SSL builder for HttpServer
pub fn tls_builder() -> SslAcceptorBuilder {
    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(".keystore/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(".keystore/cert.pem")
        .unwrap();
    builder
}
