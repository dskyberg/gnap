//! Transaction API Handlers
use crate::grant::request::process_request;
use actix_web::{web, HttpResponse};
use dao::service::Service;
use log::{error, trace};
use model::grant::GrantRequest;

/// HTTP OPTIONS <as>/gnap/tx
pub async fn grant_options(service: web::Data<Service>) -> HttpResponse {
    trace!("grant_options");
    match service.get_grant_options().await {
        Ok(data) => {
            trace!("Retrieved grant options: {:?}", data);
            HttpResponse::Ok().json(data)
        }
        Err(err) => {
            error!("{:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}


/// Initiate a grant transaction
pub async fn grant_request(
    service: web::Data<Service>,
    request: web::Json<GrantRequest>,
) -> HttpResponse {
    // Create a response from the request
    let result = process_request(&service, request.into_inner()).await;
    match result {
        Ok(data) => {
            trace!("processed grant request: {:?}", data);
            HttpResponse::Ok().json(data)
        }
        Err(err) => {
            error!("{:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use model::grant::GrantRequest;
    use serde_json;
    #[test]
    fn happy_test() {
        let re = r#"
        {
            "access_token":
                {
                    "access": ["foo", {"type": "bar", "actions":["read","write"]}],
                    "label": "my_label",
                    "flags": ["bearer", "split"]
                }

        }
        "#;
        let gr: GrantRequest = serde_json::from_str(re).expect("Failed!!");
        println!("GrantRequest: {:?}", &gr);
        assert!(true);
    }
}
