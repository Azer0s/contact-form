use std::error::Error;
use std::future::Future;
use crate::domain::contact_details::ContactDetails;

pub trait Repository {
    fn create(&self, contact_details: ContactDetails) -> impl Future<Output = Result<String, Box<dyn Error>>> + Send;
}
