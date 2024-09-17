use crate::persistence::contact_details;
use crate::service::email;

pub struct Common {
    pub contact_details_repository: Box<dyn contact_details::Repository>,
    pub email_service: Box<dyn email::Service>
}

impl Common {
    pub fn new(contact_details_repository: Box<dyn contact_details::Repository>, email_service: Box<dyn email::Service>) -> Self {
        Common {
            contact_details_repository,
            email_service
        }
    }
}