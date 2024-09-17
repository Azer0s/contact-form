use std::error::Error;
use async_trait::async_trait;
use crate::domain::email::Email;

pub mod implementation;

#[async_trait]
pub trait Service: Send + Sync {
    async fn send_email(&self, email: &Email) -> Result<(), Box<dyn Error>>;
}