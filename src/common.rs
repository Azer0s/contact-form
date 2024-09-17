use crate::persistence::contact_details;

pub struct Common {
    pub contact_details_repository: Box<dyn contact_details::repository::Repository>
}

impl Common {
    pub fn new(contact_details_repository: Box<dyn contact_details::repository::Repository>) -> Self {
        Common {
            contact_details_repository
        }
    }
}