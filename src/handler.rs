use crate::domain::contact_details::ContactDetails;
use crate::domain::serialization_error::SerializationError;
use lambda_http::{http, Body, Error, Request, Response};

pub async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    if event.method() != http::method::Method::POST {
        return Ok(Response::builder()
            .status(405)
            .body("Method Not Allowed".into())
            .map_err(Box::new)?);
    }

    let contact_details = match ContactDetails::try_from(event) {
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

    println!("Received contact details: {:?}", contact_details);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello, World!".into())
        .map_err(Box::new)?;
    Ok(resp)
}