
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GnapOptions {
    grant_request_endpoint: String,
    introspection_endpoint: String,
    resource_registration_endpoint: String,
    token_formats_supported: Vec<String>
}

impl GnapOptions {
    pub fn new() -> Self {
        GnapOptions {
            grant_request_endpoint: "https//localhost:8000/gnap/tx".to_owned(),
            introspection_endpoint: "https://localhost:8000/gnap/introspect".to_owned(),
            resource_registration_endpoint: "https://localhost:8000/gnap/resource".to_owned(),
            token_formats_supported: vec![
                "jwt".to_owned(),
                "paseto".to_owned()
            ]
        }
    }
}