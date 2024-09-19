use crate::domain::contact_details::ContactDetails;
use crate::get_from_dynamodb_response;
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

    async fn get(&self, id: &str) -> anyhow::Result<ContactDetails> {
        let resp = self.db
            .get_item()
            .table_name(&self.table_name)
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await;
        
        let resp = match resp { 
            Ok(resp) => resp,
            Err(e) => return Err(e.into()),
        };
        
        if resp.item.is_none() {
            return Err(anyhow::anyhow!("Item not found"));
        }
        
        let item = resp.item.unwrap();
        
        Ok(ContactDetails {
            email: get_from_dynamodb_response!(item, email, as_s),
            name: get_from_dynamodb_response!(item, name, as_s),
            message: get_from_dynamodb_response!(item, message, as_s),
        })
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.db
            .delete_item()
            .table_name(&self.table_name)
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await
            .map(|_| ())
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
    
    async fn get(&self, _id: &str) -> anyhow::Result<ContactDetails> {
        Ok(ContactDetails {
            email: "mock-email".to_string(),
            name: "mock-name".to_string(),
            message: "mock-message".to_string(),
        })
    }
    
    async fn delete(&self, _id: &str) -> anyhow::Result<()> {
        println!("MockRepository::delete");
        Ok(())
    }
}