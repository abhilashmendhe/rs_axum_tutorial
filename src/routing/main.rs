use crate::{errors::MyAppError, helper::run};

pub mod helper;
pub mod errors;
pub mod routes;

/* $ cargo watch -q -c -w src/ -x run */
#[tokio::main]
async fn main() -> Result<(), MyAppError> {
    run().await?;
    Ok(())
}

