use crate::domain::contact_details::ContactDetails;
use crate::service::email::Service;
use async_trait::async_trait;
use aws_sdk_sesv2::error::ProvideErrorMetadata;
use aws_sdk_sesv2::primitives::Blob;
use aws_sdk_sesv2::types::{Destination, EmailContent, RawMessage};
use lambda_http::http::Uri;
use mail_builder::MessageBuilder;

pub struct SenderMeta {
    pub sender_name: String,
    pub sender_firstname: String,
    pub sender: String,
    pub from_name: String,
    pub from: String,
    pub subject: String,
}

fn build_email(contact_details: &ContactDetails, callback_uri: &Uri, message_id: String, sender_meta: &SenderMeta) -> anyhow::Result<String> {
    let link = format!("{}://{}/{}?message_id={}",
                       callback_uri.scheme_str().unwrap_or("http"),
                       callback_uri.authority().map(|x| x.as_str()).unwrap_or("localhost"),
                       callback_uri.path(),
                       message_id);

    let html = format!("<h3>Hi, {}!</h3>", contact_details.name)
        + " <p>I'll get back to you as soon as possible.</p> "
        + &format!("<p>Please click on the <a href=\"{}\">link</a> to confirm your email address</p>", link)
        + &format!("<p>Best regards, {}</p>", sender_meta.sender_firstname);

    let eml = MessageBuilder::new()
        .from((sender_meta.from_name.clone(), sender_meta.from.clone()))
        .sender((sender_meta.sender_name.clone(), sender_meta.sender.clone()))
        .to(contact_details.email.as_str())
        .subject(sender_meta.subject.clone())
        .html_body(html)
        .write_to_string()?;

    Ok(eml)
}

pub struct MockEmailService;

#[async_trait]
impl Service for MockEmailService {
    async fn send_email(&self, contact_details: &ContactDetails, callback_uri: &Uri, message_id: String) -> anyhow::Result<()> {
        println!("MockEmailService::send_email");
        println!("{}", build_email(contact_details, callback_uri, message_id, &SenderMeta {
            sender_name: "Ariel Simulevski".to_string(),
            sender_firstname: "Ariel".to_string(),
            sender: "noreply@simulevski.at".to_string(),
            from_name: "Ariel Simulevski".to_string(),
            from: "ariel@simulevski.at".to_string(),
            subject: "Hi, there!".to_string()
        })?);
        Ok(())
    }
}

pub struct AmazonSesEmailService {
    pub(crate) client: aws_sdk_sesv2::Client,
    pub(crate) sender_meta: SenderMeta,
}

impl AmazonSesEmailService {
    pub fn new(client: aws_sdk_sesv2::Client, sender_meta: SenderMeta) -> AmazonSesEmailService {
        AmazonSesEmailService {
            client,
            sender_meta,
        }
    }
}

#[async_trait]
impl Service for AmazonSesEmailService {
    async fn send_email(&self, contact_details: &ContactDetails, callback_uri: &Uri, message_id: String) -> anyhow::Result<()> {
        let dest: Destination = Destination::builder()
            .to_addresses(contact_details.email.as_str())
            .build();

        let eml = build_email(contact_details, callback_uri, message_id, &self.sender_meta)?;
        let raw_msg = RawMessage::builder().data(Blob::new(eml)).build()?;
        let content = EmailContent::builder().raw(raw_msg).build();

        let resp = self.client
            .send_email()
            .from_email_address(self.sender_meta.sender.as_str())
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