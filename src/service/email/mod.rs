use async_trait::async_trait;
use lambda_http::http::Uri;
use crate::domain::contact_details::ContactDetails;

pub mod implementation;

#[async_trait]
pub trait Service: Send + Sync {
    async fn send_email(&self, contact_details: &ContactDetails, callback_uri: &Uri, message_id: String) -> anyhow::Result<()>;
}