use aws_sdk_dynamodb::Client;

pub struct Common {
    client: Client,
}

impl Common {
    pub fn new(client: Client) -> Common {
        Common {
            client,
        }
    }
}