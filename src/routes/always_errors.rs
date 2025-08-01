use axum::http::StatusCode;

pub async fn always_errors() -> Result<(), StatusCode> {
    // Ok(()) // 200 Ok
    // Err(StatusCode::IM_A_TEAPOT) // 418 HTTP Error
    Err(StatusCode::INTERNAL_SERVER_ERROR) // 500 HTTP Error
}