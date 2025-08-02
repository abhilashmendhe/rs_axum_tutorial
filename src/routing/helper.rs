use tokio::net::TcpListener;

use crate::{errors::MyAppError, routes::create_routes};

pub async fn run() -> Result<(), MyAppError> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    let app = create_routes().await;

    axum::serve(listener, app)
    .await?;

    Ok(())
}
