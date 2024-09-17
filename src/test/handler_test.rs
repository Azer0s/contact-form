use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use aws_config::BehaviorVersion;
use crate::domain::contact_details::ContactDetails;
use crate::handler;
use lambda_http::{http, Body, Error, Request, Response};
use crate::common::Common;

async fn setup_test<'a>() -> impl Fn(Request) -> Pin<Box<dyn Future<Output=Result<Response<Body>, Error>> + Send + 'a>> {
    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_dynamodb::Client::new(&sdk_config);
    let common = Arc::new(Common::new(client));

    move |request: Request| {
        let future = {
            let common = common.clone();
            async move {
                let function_handler = handler(&common);
                function_handler(request).await
            }
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