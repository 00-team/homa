use actix_web::error::{self, ErrorBadRequest};
use actix_web::http::header;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Query, Redirect};
use actix_web::{get, post, HttpResponse, Responder, Scope};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use utoipa::{IntoParams, OpenApi, ToSchema};

use crate::config::config;
use crate::config::Config;
use crate::docs::UpdatePaths;
use crate::models::{JsonStr, ListInput, Response, Transaction, User};
use crate::utils::{get_random_bytes, get_random_string, remove_photo, CutOff};
use crate::AppState;

type Hmac256 = Hmac<Sha256>;

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::auth")),
    paths(login_telegram),
    components(schemas(LoginTelQuery)),
    servers((url = "/auth")),
    modifiers(&UpdatePaths)
)]
pub struct ApiAuthDoc;

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

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct LoginTelQuery {
    auth_date: u64,
    first_name: String,
    id: i64,
    last_name: Option<String>,
    photo_url: Option<String>,
    username: Option<String>,
    hash: String,
}

#[utoipa::path(
    get,
    params(LoginTelQuery),
    responses((status = 302))
)]
#[get("/login-telegram/")]
async fn login_telegram(q: Query<LoginTelQuery>) -> impl Responder {
    let mut msg = format!(
        "auth_date={}\nfirst_name={}\nid={}",
        q.auth_date, q.first_name, q.id
    );

    if let Some(last_name) = &q.last_name {
        msg += &("\nlast_name=".to_string() + last_name)
    }
    if let Some(photo_url) = &q.photo_url {
        msg += &("\nphoto_url=".to_string() + photo_url)
    }
    if let Some(username) = &q.username {
        msg += &("\nusername=".to_string() + username)
    }

    let mut mac = Hmac256::new_from_slice(&config().bot_token_hash).unwrap();
    mac.update(msg.as_bytes());
    let result = mac.finalize();

    if hex::encode(result.into_bytes()) != q.hash {
        return Err(ErrorBadRequest("invalid login credentials ❌"));
    }

    Ok(HttpResponse::build(StatusCode::FOUND)
        .insert_header((header::LOCATION, "/"))
        .insert_header((header::AUTHORIZATION, "Bearer token"))
        .finish())
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
