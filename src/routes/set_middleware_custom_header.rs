use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::routes::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    mut request: Request, next: Next
) -> Result<Response, StatusCode> {

    let headers = request.headers();
    let messsage = headers.get("message")
                    .ok_or_else(|| { StatusCode::BAD_REQUEST })?;
    let message = messsage.to_str().map_err(|_error| {
        StatusCode::BAD_REQUEST
    })?.to_string();

    let extenstions = request
        .extensions_mut();

    extenstions.insert(HeaderMessage(message));

    Ok(next.run(request).await)
}