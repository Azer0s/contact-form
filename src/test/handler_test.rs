use crate::common::Common;
use crate::domain::contact_details::ContactDetails;
use crate::handler;
use lambda_http::{http, Body, Error, Request, Response};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

async fn setup_test() -> impl Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response<Body>, Error>> + Send>> {
    let repository = crate::persistence::contact_details::implementation::MockRepository;
    let common = Arc::new(Box::new(Common::new(repository)));

    move |request: Request| {
        let common = Arc::clone(&common);

        let future = async move {
            let function_handler = handler(&common);
            function_handler(request).await
        };

        Box::pin(future)
    }
}

#[tokio::test]
async fn test_contact_form() {
    let input = serde_json::to_string(&ContactDetails {
        name: "John Doe".to_string(),
        email: "john@doe.com".to_string(),
        message: "Hello, World!".to_string(),
    }).expect("failed to serialize input");

    let req  = http::request::Builder::new()
        .method(http::method::Method::POST)
        .body(Body::from(input))
        .expect("failed to build request");
    
    let function_handler = setup_test().await;
    let resp = function_handler(req).await.expect("failed to execute handler");
    println!("{:?}", resp);
}