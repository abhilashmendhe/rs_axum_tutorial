
use axum::{
    extract::FromRequest, 
    http::StatusCode, 
    Json, 
    RequestExt,
};
use validator::Validate;

use crate::routes::validate_with_serde::RequestUser;

impl<S> FromRequest<S> for RequestUser 
where 
    S: Send + Sync {
    type Rejection = (StatusCode, String);
    async  fn from_request(req:axum::extract::Request, _state: &S,) -> Result<Self,Self::Rejection> {
        let Json(user) = req
                                    .extract::<Json<RequestUser>, _>()
                                    .await
                                    .map_err(|err| (StatusCode::BAD_REQUEST, format!("{}",err)))?;
        if let Err(err) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}",err)))
        }
        
        Ok(user)
    }
}

pub async fn custom_json_extractor(user_ctx: RequestUser) {
    dbg!(user_ctx);
}

/*

Will see FromRequestParts in the future....

// impl<S> FromRequestParts<S> for RequestUser 
// where 
//     S: Send + Sync
// {
//     type Rejection = (StatusCode, String);
//     async fn from_request_parts(
//         parts: &mut axum::http::request::Parts, 
//         _state: &S,) -> Result<Self,Self::Rejection> {
//             // let Json(user) = parts
//             //                             .extract::<Json<RequestUser>>()
//             //                             .await
//             //                             .map_err(|err| (StatusCode::BAD_REQUEST, format!("{}",err)))?;

//             let Json(user) = parts
//                 .extensions
//                 .get::<Json<RequestUser>>()
//                 .ok_or((StatusCode::BAD_REQUEST, "RequestUser context not found".to_string()))?
//                 .clone();
//             // let user = parts
//             //     .extensions
//             //     .get::<RequestUser>()
//             //     .ok_or((StatusCode::BAD_REQUEST, "RequestUser context not found".to_string()))?
//             //     .clone();
//             Ok(user)
//     }
// }

// pub async fn inject_user_ctx(mut req: Request<Body>, next: Next) -> impl IntoResponse {

//     dbg!(&req.body());
    
//     let username = req
//                     .headers()
//                     .get("username")
//                     .and_then(|v| v.to_str().ok())
//                     .map(String::from)
//                     .unwrap();
//     let password = req
//                     .headers()
//                     .get("password")
//                     .and_then(|v| v.to_str().ok())
//                     .map(String::from)
//                     .unwrap();
    
//     let user_ctx = RequestUser {username, password};

//     req.extensions_mut().insert(user_ctx);

//     next.run(req).await
// }
*/

