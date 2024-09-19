use crate::common::Common;
use crate::controller::confirm_message::func as confirm_message;
use crate::controller::create_message::func as create_message;
use http::method;
use lambda_http::{http, Body, Error, Request, Response};
use method::Method;
use std::future::Future;
use std::pin::Pin;

pub fn handler<'a>(common: &'a Common) -> impl Fn(Request) -> Pin<Box<dyn Future<Output=Result<Response<Body>, Error>> + Send + 'a>> + 'a {
    move |request: Request| {
        let future = function_handler(common, request);
        Box::pin(future)
    }
}

async fn function_handler(common: &Common, event: Request) -> Result<Response<Body>, Error> {
    match *event.method() {
        Method::POST => create_message(common, event).await,
        Method::GET => confirm_message(common, event).await,
        _ => Ok(Response::builder()
            .status(405)
            .body("Method Not Allowed".into())
            .map_err(Box::new)?),
    }
}