use crate::domain::contact_details::ContactDetails;
use crate::service::email::Service;
use async_trait::async_trait;
use aws_sdk_sesv2::error::ProvideErrorMetadata;
use aws_sdk_sesv2::primitives::Blob;
use aws_sdk_sesv2::types::{Destination, EmailContent, RawMessage};
use mail_builder::MessageBuilder;

const EMAIL_SUBJECT: &str = "Hi there!";
const EMAIL_SENDER: &str = "noreply@simulevski.at";
const EMAIL_SENDER_NAME: &str = "Ariel Simulevski";
const EMAIL_SENDER_FIRST_NAME: &str = "Ariel";

fn build_email(contact_details: &ContactDetails, message_id: String) -> anyhow::Result<String> {
    let link = format!("https://simulevski.at/confirm/{}", message_id);
    
    let html = format!("<h3>Hi, {}!</h1>", contact_details.name)
        + " <p>I'll get back to you as soon as possible.</p> "
        + &format!("<p>Please click on the <a href=\"{}\">link</a> to confirm your email address</p>", link)
        + &format!("<p>Best regards, {}</p>", EMAIL_SENDER_FIRST_NAME);

    let eml = MessageBuilder::new()
        .from((EMAIL_SENDER_NAME, EMAIL_SENDER))
        .sender((EMAIL_SENDER_NAME, EMAIL_SENDER))
        .to(contact_details.email.as_str())
        .subject(EMAIL_SUBJECT)
        .html_body(html)
        .write_to_string()?;
    
    Ok(eml)
}

pub struct MockEmailService;

#[async_trait]
impl Service for MockEmailService {
    async fn send_email(&self, contact_details: &ContactDetails, message_id: String) -> anyhow::Result<()> {
        println!("MockEmailService::send_email");
        println!("{}", build_email(contact_details, message_id)?);
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
    async fn send_email(&self, contact_details: &ContactDetails, message_id: String) -> anyhow::Result<()> {
        let dest: Destination = Destination::builder()
            .to_addresses(contact_details.email.as_str())
            .build();
        
        let eml = build_email(contact_details, message_id)?;
        let raw_msg = RawMessage::builder().data(Blob::new(eml)).build()?;
        let content = EmailContent::builder().raw(raw_msg).build();

        let resp = self.client
            .send_email()
            .from_email_address("noreply@simulevski.at")
            .destination(dest)
            .content(content)
            .send()
            .await;
        
        if let Err(e) = &resp {
            println!("Error: {}", e);
            if let Some(err) = e.message() {
                println!("Caused by: {}", err);
            }
            
            return Err(anyhow::anyhow!("Failed to send email"));
        }

        if resp?.message_id.is_some() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to send email"))
        }
    }
}