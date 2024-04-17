use actix_multipart::form::MultipartForm;
use actix_web::web::{Data, Json, Query};
use actix_web::{
    delete, get, patch, post, put, HttpResponse, Responder, Scope,
};
use serde::{Deserialize, Serialize};
// use sha2::{Digest, Sha512};
use utoipa::{OpenApi, ToSchema};

// use crate::api::verification::verify;
// use crate::config::Config;
// use crate::docs::UpdatePaths;
// use crate::models::{
//     Action, Address, JsonStr, ListInput, Response, Transaction, UpdatePhoto,
//     User,
// };
// use crate::utils::{
//     get_random_bytes, get_random_string, remove_photo, save_photo, sql_unwrap,
//     CutOff,
// };
use crate::AppState;

// #[derive(OpenApi)]
// #[openapi(
//     tags((name = "api::user")),
//     paths(
//         login, user_get, user_update, user_update_photo,
//         user_delete_photo, user_wallet_test, user_transactions_list
//     ),
//     components(schemas(
//         User, LoginBody, UserUpdateBody, Address, UpdatePhoto, Transaction
//     )),
//     servers((url = "/auth")),
//     modifiers(&UpdatePaths)
// )]
// pub struct ApiUserDoc;
//
// #[derive(Debug, Deserialize, ToSchema)]
// struct LoginBody {
//     phone: String,
//     code: String,
// }
//
// #[utoipa::path(
//     post,
//     request_body = LoginBody,
//     responses(
//         (status = 200, body = User),
//         (status = 400, body = String)
//     )
// )]
// /// Login
// #[post("/login/")]
// async fn login(body: Json<LoginBody>, state: Data<AppState>) -> Response<User> {
//     verify(&body.phone, &body.code, Action::Login, &state.sql).await?;
//
//     let token = get_random_string(Config::TOKEN_ABC, 69);
//     let token_hashed = hex::encode(Sha512::digest(&token));
//
//     let result = sqlx::query_as! {
//         User,
//         "select * from users where phone = ?",
//         body.phone
//     }
//     .fetch_one(&state.sql)
//     .await;
//
//     let user: User = match result {
//         Ok(mut v) => {
//             v.token = token;
//
//             let _ = sqlx::query_as! {
//                 User,
//                 "update users set token = ? where id = ?",
//                 token_hashed, v.id
//             }
//             .execute(&state.sql)
//             .await;
//
//             v
//         }
//         Err(_) => {
//             let result = sqlx::query_as! {
//                 User,
//                 "insert into users (phone, token) values(?, ?)",
//                 body.phone, token_hashed
//             }
//             .execute(&state.sql)
//             .await;
//
//             User {
//                 phone: body.phone.clone(),
//                 token,
//                 id: result.unwrap().last_insert_rowid(),
//                 ..Default::default()
//             }
//         }
//     };
//
//     Ok(Json(user))
// }

#[derive(Debug, Deserialize)]
pub struct LoginTelQuery {
    hash: String
}

#[get("/login-telegram/")]
async fn login_telegram(
    query: Query<LoginTelQuery>, state: Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().body(format!("{query:#?}"))
}

pub fn router() -> Scope {
    Scope::new("/auth").service(login_telegram)
    // .service(user_get)
    // .service(user_update)
    // .service(user_update_photo)
    // .service(user_delete_photo)
    // .service(user_wallet_test)
    // .service(user_transactions_list)
}
