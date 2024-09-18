use crate::common::Common;
use crate::domain::contact_details::ContactDetails;
use crate::domain::serialization_error::SerializationError;
use lambda_http::{Body, Error, Request, Response};

pub async fn func(common: &Common, event: Request) -> Result<Response<Body>, Error> {
    let contact_details = match ContactDetails::try_from(&event) {
        Ok(contact_details) => contact_details,
        Err(SerializationError::NameEmpty) => {
            return Ok(Response::builder()
                .status(400)
                .body("Name cannot be empty".into())
                .map_err(Box::new)?);
        },
        Err(err) => {
            return Ok(Response::builder()
                .status(400)
                .body(format!("Invalid request: {}", err).into())
                .map_err(Box::new)?);
        },
    };

    let resp = common.contact_details_repository.create(&contact_details).await;

    if let Err(e) = resp {
        println!("Error: {}", e);
        if let Some(err) = e.source() {
            println!("Caused by: {}", err);
        }

        return Ok(Response::builder()
            .status(500)
            .body(format!("Internal Server Error: {}", e).into())
            .map_err(Box::new)?);
    }
    
    if let Err(e) = common.email_service.send_email(&contact_details, event.uri(), resp?).await {
        return Ok(Response::builder()
            .status(500)
            .body(format!("Internal Server Error: {}", e).into())
            .map_err(Box::new)?);
    }

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Message sent successfully".into())
        .map_err(Box::new)?)
}