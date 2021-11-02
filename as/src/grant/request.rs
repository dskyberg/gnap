use model::grant::*;
use errors::GnapError;
use uuid::Uuid;
use dao::Service;
use log::trace;

pub async fn process_request(service: &Service, request: GrantRequest) -> Result<GrantResponse, GnapError> {

    let tx = service.start_transaction(request.clone()).await?;
    let uri = format!("http://locahost:8000/tx/{}", &tx.tx_id);
    let rc = RequestContinuation::as_uri(&uri.clone());
    let mut interact_response = InteractResponse {
        tx_continue: rc,
        redirect: None
    };

    // What are the interaction methods?
    for method in  request.interact.unwrap().start.iter() {
        match method {
            InteractStartMode::Redirect => {
                trace!("GrantRequest interaction contains Redirect");
                interact_response.redirect = Some(uri.clone());
            },
            InteractStartMode:: App => {
                trace!("GrantRequest interaction contains App");
            },
            InteractStartMode::UserCode => {
                trace!("GrantRequest interaction contains UserCode");
            }
        }
    }

    let response = GrantResponse{
        instance_id: tx.tx_id.clone(),
        interact: Some(interact_response)
    };

    Ok(response)

}