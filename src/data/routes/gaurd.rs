use axum::{body::Body, http::{Request, StatusCode}, middleware::Next, response::Response};
use headers::{authorization::Bearer, Authorization, HeaderMapExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::users;
use crate::database::users::Entity as Users;

pub async fn gaurd(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {

    let token = request
                .headers()
                .typed_get::<Authorization<Bearer>>()
                .ok_or(StatusCode::BAD_REQUEST)?
                .token()
                .to_owned();
    let database = request
                    .extensions()
                    .get::<DatabaseConnection>()
                    .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = Users::find()
                .filter(users::Column::Token.eq(Some(token)))
                .one(database)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let Some(user) = user else {return Err(StatusCode::UNAUTHORIZED)};
    request
        .extensions_mut()
        .insert(user);

    Ok(next.run(request).await)
}