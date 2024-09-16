use crate::domain::contact_details::ContactDetails;
use crate::function_handler;
use lambda_http::{http, Body};

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

    let resp = function_handler(req).await.expect("failed to execute handler");
    println!("{:?}", resp);
}