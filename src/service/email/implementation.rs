use crate::domain::email::Email;
use crate::service::email::Service;
use async_trait::async_trait;
use std::error::Error;

pub struct MockEmailService;

#[async_trait]
impl Service for MockEmailService {
    async fn send_email(&self, email: &Email) -> Result<(), Box<dyn Error>> {
        println!("{}", format!("{}", email));
        println!("MockEmailService::send_email");
        Ok(())
    }
}