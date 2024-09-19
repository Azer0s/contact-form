use crate::common::Common;
use lambda_http::{Body, Error, Request, Response};
use url::Url;
use crate::{try_or_status_code};

fn bad_request() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(400)
        .body("Bad Request".into())
        .map_err(Box::new)?)
}

pub async fn func(common: &Common, event: Request) -> Result<Response<Body>, Error> {
    let url = match Url::parse(format!("{}", event.uri()).as_str()) {
        Ok(url) => url,
        Err(_) => return bad_request(),
    };

    let message_id = match url
        .query_pairs()
        .find(|(key, _)| key == "message_id") {
        Some((_, message_id)) => message_id,
        None => return bad_request(),
    };
    
    let msg = common.contact_details_repository.get(&message_id).await;
    let msg = try_or_status_code!(msg, 500);
    
    let resp = common.contact_details_repository.delete(&message_id).await;
    try_or_status_code!(resp, 500);
    
    let resp = common.email_service.send_message_email_to_receiver(&msg).await;
    try_or_status_code!(resp, 500);
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Message confirmed".into())
        .map_err(Box::new)?)
}