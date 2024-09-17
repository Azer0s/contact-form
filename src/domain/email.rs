use std::fmt::Display;
use crate::domain::contact_details::ContactDetails;

pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email {{ to: {}, subject: {}, body: {} }}", self.to, self.subject, self.body)
    }
}

impl From<&ContactDetails> for Email {
    fn from(contact_details: &ContactDetails) -> Self {
        Email {
            to: contact_details.email.clone(),
            subject: format!("Hi, {}!", contact_details.name),
            body: format!("Hi, {}! I'll get back to you as soon as possible. Please click on the link to confirm your email address: {}", contact_details.name, contact_details.email)
        }
    }
}