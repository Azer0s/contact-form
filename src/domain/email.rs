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

impl Email {
    pub fn new(contact_details: &ContactDetails, subject: String, body: String) -> Email {
        Email {
            to: contact_details.email.clone(),
            subject,
            body
        }
    }
}
