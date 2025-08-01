use axum_extra::TypedHeader;
use headers::UserAgent;


pub async fn mirror_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}