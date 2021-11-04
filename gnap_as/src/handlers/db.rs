use dao::service::Service;
use uuid::Uuid;
use actix_web::{web, HttpResponse};

use model::client::GnapClientRequest;
use log::{trace, error};

pub async fn get_client(
    service: web::Data<Service>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    trace!("get_client: {:?}", id);
    let id = id.into_inner();
    match service.get_client(&id).await
    {
        Ok(Some(data)) => {
            trace!("Retrieved client: {:?}", data);
            HttpResponse::Ok().json(data)
        },
        Ok(None) => {
            trace!("client not found");
            HttpResponse::NotFound()
            .body(format!("No client found with id {}", id.to_string()))
        },
        Err(err) => {
            error!("{:?}",err);
            HttpResponse::InternalServerError().body(err.to_string())
        },
    }
}

pub async fn add_client(
    service: web::Data<Service>,
    client: web::Json<GnapClientRequest>
) -> HttpResponse {

    match service.add_client(client.into_inner()).await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}