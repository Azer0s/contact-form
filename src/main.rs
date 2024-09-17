mod handler;
mod domain;
mod test;
mod common;
mod controller;
mod persistence;

use std::sync::Arc;
use aws_config::BehaviorVersion;
use crate::handler::handler;
use lambda_http::{run, service_fn, tracing, Error};
use crate::persistence::contact_details;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_dynamodb::Client::new(&sdk_config);
    let db = contact_details::implementation::DynamoDbRepository::new(Arc::new(client));
    let common = common::Common::new(Box::new(db));

    run(service_fn(handler(&common))).await
}
