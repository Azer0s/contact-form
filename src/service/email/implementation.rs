use crate::domain::email::Email;
use crate::service::email::Service;
use async_trait::async_trait;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};

pub struct MockEmailService;

#[async_trait]
impl Service for MockEmailService {
    async fn send_email(&self, email: &Email) -> anyhow::Result<()> {
        println!("MockEmailService::send_email");
        println!("{}", email);
        Ok(())
    }
}

pub struct AmazonSesEmailService {
    pub(crate) client: aws_sdk_sesv2::Client,
}

impl AmazonSesEmailService {
    pub fn new(client: aws_sdk_sesv2::Client) -> AmazonSesEmailService {
        AmazonSesEmailService {
            client
        }
    }
}

#[async_trait]
impl Service for AmazonSesEmailService {
    async fn send_email(&self, email: &Email) -> anyhow::Result<()> {
        let mut dest: Destination = Destination::builder().build();
        dest.to_addresses = Some(vec![email.to.clone()]);
        
        let subject_content = Content::builder()
            .data(email.subject.clone())
            .charset("UTF-8")
            .build()
            .expect("building Content");
        
        let body_content = Content::builder()
            .data(email.body.clone())
            .charset("UTF-8")
            .build()
            .expect("building Content");
        let body = Body::builder().text(body_content).build();

        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();

        let email_content = EmailContent::builder().simple(msg).build();

        let resp = self.client
            .send_email()
            .from_email_address("noreply@simulevski.at")
            .destination(dest)
            .content(email_content)
            .send()
            .await?;
        
        if resp.message_id.is_some() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to send email"))
        }
    }
}