use axum::Extension;

#[derive(Debug, Clone)]
pub struct HeaderMessage(pub String);

pub async fn read_middleware_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}