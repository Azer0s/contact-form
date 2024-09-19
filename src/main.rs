mod handler;
mod domain;
mod test;
mod common;
mod controller;
mod persistence;
mod service;

use std::env;
use crate::handler::handler;
use crate::persistence::contact_details;
use aws_config::BehaviorVersion;
use lambda_http::{run, service_fn, tracing, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let table_name = env::var("TABLE_NAME").unwrap_or("contact_form_messages".to_string());
    let sender_meta = service::email::implementation::SenderMeta {
        sender_name: env::var("SENDER_NAME")?,
        sender_firstname: env::var("SENDER_FIRSTNAME")?,
        sender: env::var("SENDER_EMAIL")?,
        subject: env::var("EMAIL_SUBJECT")?,
        from_name: env::var("FROM_NAME").unwrap_or(env::var("SENDER_NAME")?),
        from: env::var("FROM_EMAIL").unwrap_or(env::var("SENDER_EMAIL")?),
        receiver: env::var("RECEIVER_EMAIL")?,
    };

    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let db = contact_details::implementation::DynamoDbRepository::new(aws_sdk_dynamodb::Client::new(&sdk_config), table_name);
    let email = service::email::implementation::AmazonSesEmailService::new(aws_sdk_sesv2::Client::new(&sdk_config), sender_meta);
    
    let common = common::Common::new(Box::new(db), Box::new(email));

    run(service_fn(handler(&common))).await
}
