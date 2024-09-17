use crate::common::Common;
use crate::controller::create_message::func as create_message;
use lambda_http::{http, Body, Error, Request, Response};
use std::future::Future;
use std::pin::Pin;

pub fn handler<'a>(common: &'a Common) -> impl Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response<Body>, Error>> + Send + 'a>> + 'a {
    move |request: Request| {
        let future = function_handler(common, request);
        Box::pin(future)
    }
}

async fn function_handler(common: &Common, event: Request) -> Result<Response<Body>, Error> {
    match event.method() { 
        &http::method::Method::POST => create_message(common, event).await,
        _ => Ok(Response::builder()
            .status(405)
            .body("Method Not Allowed".into())
            .map_err(Box::new)?),
    }
}