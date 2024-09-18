use lambda_http::Request;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::domain::serialization_error::SerializationError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactDetails {
    pub name: String,
    pub email: String,
    pub message: String,
}

impl TryFrom<&Request> for ContactDetails {
    type Error = SerializationError;

    fn try_from(req: &Request) -> Result<Self, Self::Error> {
        let body = req.body();
        let contact_details: Result<ContactDetails, _> = serde_json::from_slice(body.as_ref());

        let contact_details = match contact_details {
            Ok(contact_details) => contact_details,
            Err(_) => return Err(SerializationError::InvalidJson),
        };

        let name = match contact_details.name {
            n if n.is_empty() => return Err(SerializationError::NameEmpty),
            n if n.len() > 100 => return Err(SerializationError::NameTooLong),
            n => n,
        };

        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        let email = match contact_details.email {
            e if e.is_empty() => return Err(SerializationError::EmailEmpty),
            e if e.len() > 100 => return Err(SerializationError::EmailTooLong),
            e if !email_regex.is_match(&e) => return Err(SerializationError::EmailInvalid),
            e => e,
        };

        let message = match contact_details.message {
            m if m.is_empty() => return Err(SerializationError::MessageEmpty),
            m if m.len() > 1000 => return Err(SerializationError::MessageTooLong),
            message => message,
        };

        Ok(ContactDetails { name, email, message })
    }
}
