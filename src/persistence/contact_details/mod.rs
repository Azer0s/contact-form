use async_trait::async_trait;
use crate::domain::contact_details::ContactDetails;

pub mod implementation;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, contact_details: &ContactDetails) -> anyhow::Result<String>;
    async fn get(&self, id: &str) -> anyhow::Result<ContactDetails>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
}
