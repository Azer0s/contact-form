use crate::persistence::contact_details;

pub struct Common<T: contact_details::repository::Repository> {
    pub contact_details_repository: T
}

impl<T: contact_details::repository::Repository> Common<T> {
    pub fn new(contact_details_repository: T) -> Common<T> {
        Common {
            contact_details_repository
        }
    }
}