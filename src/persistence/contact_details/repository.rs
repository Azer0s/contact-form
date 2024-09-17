use std::error::Error;
use async_trait::async_trait;
use crate::domain::contact_details::ContactDetails;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, contact_details: ContactDetails) -> Result<String, Box<dyn Error>>;
}
