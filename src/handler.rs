use std::future::Future;
use std::pin::Pin;
use crate::domain::contact_details::ContactDetails;
use crate::domain::serialization_error::SerializationError;
use lambda_http::{http, Body, Error, Request, Response};
use crate::common::Common;

pub fn handler<'a>(common: &'a Common) -> impl Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response<Body>, Error>> + Send + 'a>> + 'a {
    move |request: Request| {
        let future = function_handler(common, request);
        Box::pin(future)
    }
}

async fn function_handler(_common: &Common, event: Request) -> Result<Response<Body>, Error> {
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