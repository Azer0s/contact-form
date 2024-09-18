mod handler;
mod domain;
mod test;
mod common;
mod controller;
mod persistence;
mod service;

use crate::handler::handler;
use crate::persistence::contact_details;
use aws_config::BehaviorVersion;
use lambda_http::{run, service_fn, tracing, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let db = contact_details::implementation::DynamoDbRepository::new(aws_sdk_dynamodb::Client::new(&sdk_config));
    let email = service::email::implementation::AmazonSesEmailService::new(aws_sdk_sesv2::Client::new(&sdk_config));
    
    let common = common::Common::new(Box::new(db), Box::new(email));

    run(service_fn(handler(&common))).await
}
