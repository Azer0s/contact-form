use std::fmt::{Display, Formatter};

pub enum SerializationError {
    NameEmpty,
    NameTooLong,
    EmailEmpty,
    EmailInvalid,
    EmailTooLong,
    MessageEmpty,
    MessageTooLong,
    InvalidJson,
}

impl Display for SerializationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializationError::NameEmpty => write!(f, "Name cannot be empty"),
            SerializationError::NameTooLong => write!(f, "Name is too long"),
            SerializationError::EmailEmpty => write!(f, "Email cannot be empty"),
            SerializationError::EmailInvalid => write!(f, "Email is invalid"),
            SerializationError::EmailTooLong => write!(f, "Email is too long"),
            SerializationError::MessageEmpty => write!(f, "Message cannot be empty"),
            SerializationError::MessageTooLong => write!(f, "Message is too long"),
            SerializationError::InvalidJson => write!(f, "Invalid JSON"),
        }
    }
}