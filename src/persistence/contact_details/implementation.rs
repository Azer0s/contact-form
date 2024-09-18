use crate::domain::contact_details::ContactDetails;
use crate::persistence::contact_details::Repository;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

const DYNAMO_DB_TABLE_NAME: &str = "contact_form_messages";

pub struct DynamoDbRepository {
    pub(crate) db: Client,
}

impl DynamoDbRepository {
    pub fn new(db: Client) -> DynamoDbRepository {
        DynamoDbRepository {
            db
        }
    }
}

#[async_trait]
impl Repository for DynamoDbRepository {
    async fn create(&self, contact_details: &ContactDetails) -> anyhow::Result<String> {
        let id = uuid::Uuid::new_v4().to_string();

        self.db.
            put_item()
            .table_name(DYNAMO_DB_TABLE_NAME.to_string())
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