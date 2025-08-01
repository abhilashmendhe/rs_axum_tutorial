use axum_playlist::{errors::MyAppError, run};

/* $ cargo watch -q -c -w src/ -x run */
#[tokio::main]
async fn main() -> Result<(), MyAppError> {
    run().await?;
    Ok(())
}

