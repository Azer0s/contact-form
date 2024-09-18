use crate::domain::contact_details::ContactDetails;
use crate::persistence::contact_details::Repository;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

pub struct DynamoDbRepository {
    pub(crate) db: Client,
    pub(crate) table_name: String,
}

impl DynamoDbRepository {
    pub fn new(db: Client, table_name: String) -> DynamoDbRepository {
        DynamoDbRepository {
            db,
            table_name,
        }
    }
}

#[async_trait]
impl Repository for DynamoDbRepository {
    async fn create(&self, contact_details: &ContactDetails) -> anyhow::Result<String> {
        let id = uuid::Uuid::new_v4().to_string();

        self.db.
            put_item()
            .table_name(&self.table_name)
            .item("id", AttributeValue::S(id.clone()))
            .item("email", AttributeValue::S(contact_details.email.clone()))
            .item("name", AttributeValue::S(contact_details.name.clone()))
            .item("message", AttributeValue::S(contact_details.message.clone()))
            .send()
            .await
            .map(|_| id.clone())
            .map_err(|e| e.into())
    }
}

pub struct MockRepository;

#[async_trait]
impl Repository for MockRepository {
    async fn create(&self, _contact_details: &ContactDetails) -> anyhow::Result<String> {
        println!("MockRepository::create");
        Ok("mock-id".to_string())
    }
}