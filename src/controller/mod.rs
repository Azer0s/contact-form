pub mod create_message;
pub mod confirm_message;

#[macro_export] 
macro_rules! try_or_status_code {
    ($e:expr, $status_code:expr) => {
        match $e {
            Ok(val) => val,
            Err(e) => {
                println!("Error: {}", e);
                if let Some(err) = e.source() {
                    println!("Caused by: {}", err);
                }

                return Ok(Response::builder()
                    .status($status_code)
                    .body(format!("Internal Server Error: {}", e).into())
                    .map_err(Box::new)?);
            }
        }
    };
}

#[macro_export]
macro_rules! get_from_dynamodb_response {
    ($item:expr, $e:ident, $as_x:ident) => {
        match $item.get(stringify!($e)) {
            Some($e) => $e.$as_x().unwrap().to_string(),
            None => return Err(anyhow::anyhow!(format!("{} not found!", stringify!($e)))),
        }
    };
}