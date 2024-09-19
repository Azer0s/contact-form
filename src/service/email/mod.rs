use async_trait::async_trait;
use lambda_http::http::Uri;
use crate::domain::contact_details::ContactDetails;

pub mod implementation;

#[async_trait]
pub trait Service: Send + Sync {
    async fn send_confirmation_email_to_sender(&self, contact_details: &ContactDetails, callback_uri: &Uri, message_id: String) -> anyhow::Result<()>;
    async fn send_message_email_to_receiver(&self, contact_details: &ContactDetails) -> anyhow::Result<()>;
}