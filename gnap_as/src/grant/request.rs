use model::{GnapID, grant::*};
use errors::GnapError;
use dao::service::Service;
use log::{trace, error};

pub async fn process_request(service: &Service, request: GrantRequest) -> Result<GrantResponse, GnapError> {

    // A valid request?
    if request.client.is_none() {
        // No client identifier
        error!("No client id in grant request");
        return Err(GnapError::BadData);
    }
    // This will fail if either there is no client_id in the request, or the
    // client_id is not a valid uuid.
    trace!("getting id from reqeust...");
    let client_id = request.parse_id()?;
    trace!("parsed id from request: {}", client_id.to_string());
    // This will fail if the client_id provided in the request is not found.
    let _client = service.get_client(&client_id).await?.unwrap();

    // At this point, we have determined that the request contains a valid client_id
    // and the client data was found.  Now we can compare request data against
    // the authorized client.

    // Verify the request data against client config, etc.

    // Start a transaction
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