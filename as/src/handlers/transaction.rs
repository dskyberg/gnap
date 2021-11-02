use actix_web::{web, HttpResponse};
use log::{trace, error};
use dao::Service;
use model::grant::{GrantRequest};
use crate::grant::request::process_request;

/*
pub async fn grant_options(service: Arc<Service>) -> Result<impl warp::Reply, warp::Rejection> {

    //let options = GrantOptions::new();
    let options: TransactionOptions = service.get_grant_options().await?;
    Ok(warp::reply::json(&options))
}

pub async fn grant_post(request: GrantRequest, _service: Arc<Service>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&request))
}
*/

pub async fn grant_options(
    service: web::Data<Service>
) -> HttpResponse {
    trace!("grant_options");
    match service.get_grant_options().await
    {
        Ok(data) => {
            trace!("Retrieved grant options: {:?}", data);
            HttpResponse::Ok().json(data)
        },
        Err(err) => {
            error!("{:?}",err);
            HttpResponse::InternalServerError().body(err.to_string())
        },
    }
}

/// Initiate a grant transaction
pub async fn grant_request(
    service: web::Data<Service>,
    request: web::Json<GrantRequest>
) -> HttpResponse {
        trace!("grant_request: {:?}", &request);
        // Create a response from the request
        let response = process_request(&service, request.into_inner()).await.unwrap();

        HttpResponse::Ok().json(&response)
}

#[cfg(test)]
mod tests {
    use model::transaction::{GrantRequest};
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
        let gr:GrantRequest = serde_json::from_str(re).expect("Failed!!");
        println!("GrantRequest: {:?}", &gr);
        assert!(true);
    }
}
