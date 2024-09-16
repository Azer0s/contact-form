mod handler;
mod domain;
mod test;

use crate::handler::function_handler;
use lambda_http::{run, service_fn, tracing, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
