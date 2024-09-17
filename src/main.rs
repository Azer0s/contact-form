mod handler;
mod domain;
mod test;
mod common;

use aws_config::BehaviorVersion;
use crate::handler::handler;
use lambda_http::{run, service_fn, tracing, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_dynamodb::Client::new(&sdk_config);
    let common = common::Common::new(client);

    run(service_fn(handler(&common))).await
}
