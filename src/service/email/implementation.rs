use crate::domain::contact_details::ContactDetails;
use crate::service::email::Service;
use async_trait::async_trait;
use aws_sdk_sesv2::error::ProvideErrorMetadata;
use aws_sdk_sesv2::primitives::Blob;
use aws_sdk_sesv2::types::{Destination, EmailContent, RawMessage};
use lambda_http::http::Uri;
use mail_builder::headers::content_type::ContentType;
use mail_builder::mime::MimePart;
use mail_builder::MessageBuilder;

pub struct SenderMeta {
    pub sender_name: String,
    pub sender_firstname: String,
    pub sender: String,
    pub from_name: String,
    pub from: String,
    pub subject: String,
    pub receiver: String,
}

fn make_mail_header(sender_meta: &SenderMeta) -> MessageBuilder {
    MessageBuilder::new()
        .from((sender_meta.from_name.clone(), sender_meta.from.clone()))
        .sender((sender_meta.sender_name.clone(), sender_meta.sender.clone()))
}

fn build_confirmation_email_for_sender(contact_details: &ContactDetails, callback_uri: &Uri, message_id: String, sender_meta: &SenderMeta) -> anyhow::Result<String> {
    let link = format!("{}://{}/{}?message_id={}",
                       callback_uri.scheme_str().unwrap_or("http"),
                       callback_uri.authority().map(|x| x.as_str()).unwrap_or("localhost"),
                       callback_uri.path().trim_start_matches('/'),
                       message_id);

    let html = format!("<h3>Hi, {}!</h3>", contact_details.name)
        + " <p>I'll get back to you as soon as possible.</p> "
        + &format!("<p>Please click on the <a href=\"{}\">link</a> to confirm your email address</p>", link)
        + &format!("<p>Best regards, {}</p>", sender_meta.sender_firstname);

    let eml = make_mail_header(sender_meta)
        .to(contact_details.email.as_str())
        .subject(sender_meta.subject.clone())
        .html_body(html)
        .write_to_string()?;

    Ok(eml)
}

fn build_message_email_for_receiver(contact_details: &ContactDetails, sender_meta: &SenderMeta) -> anyhow::Result<String> {
    let body = format!("Mail from {} ({}):\n\n{}", contact_details.name, contact_details.email, contact_details.message);

    let eml = make_mail_header(sender_meta)
        .to(sender_meta.receiver.as_str())
        .subject(format!("New message from {}", contact_details.name))
        .body(MimePart::new(ContentType::new("text/plain"), body))
        .write_to_string()?;

    Ok(eml)
}

pub struct MockEmailService;

#[async_trait]
impl Service for MockEmailService {
    async fn send_confirmation_email_to_sender(&self, contact_details: &ContactDetails, callback_uri: &Uri, message_id: String) -> anyhow::Result<()> {
        println!("MockEmailService::send_email");
        println!("{}", build_confirmation_email_for_sender(contact_details, callback_uri, message_id, &SenderMeta {
            sender_name: "Ariel Simulevski".to_string(),
            sender_firstname: "Ariel".to_string(),
            sender: "noreply@simulevski.at".to_string(),
            from_name: "Ariel Simulevski".to_string(),
            from: "ariel@simulevski.at".to_string(),
            subject: "Hi, there!".to_string(),
            receiver: "ariel@simulevski.at".to_string(),
        })?);
        Ok(())
    }

    async fn send_message_email_to_receiver(&self, _contact_details: &ContactDetails) -> anyhow::Result<()> {
        println!("MockEmailService::send_message_to_receiver");
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

    async fn send_email(&self, to: String, eml: String) -> anyhow::Result<()> {
        let dest: Destination = Destination::builder()
            .to_addresses(to)
            .build();

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

#[async_trait]
impl Service for AmazonSesEmailService {
    async fn send_confirmation_email_to_sender(&self, contact_details: &ContactDetails, callback_uri: &Uri, message_id: String) -> anyhow::Result<()> {
        let eml = build_confirmation_email_for_sender(contact_details, callback_uri, message_id, &self.sender_meta)?;
        self.send_email(contact_details.email.to_string(), eml).await
    }

    async fn send_message_email_to_receiver(&self, contact_details: &ContactDetails) -> anyhow::Result<()> {
        let eml = build_message_email_for_receiver(contact_details, &self.sender_meta)?;
        self.send_email(self.sender_meta.receiver.to_string(), eml).await
    }
}