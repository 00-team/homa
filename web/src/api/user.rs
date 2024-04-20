use actix_web::cookie::{time::Duration, Cookie, SameSite};
use actix_web::http::header;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Query};
use actix_web::{get, HttpResponse, Scope};
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::{Digest, Sha256, Sha512};
use utoipa::{IntoParams, OpenApi, ToSchema};

use crate::config::config;
use crate::config::Config;
use crate::docs::UpdatePaths;
use crate::models::{AppErr, AppErrBadRequest, Response, User};
use crate::utils::{get_random_string, now, save_photo};
use crate::AppState;

type Hmac256 = Hmac<Sha256>;

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::user")),
    paths(user_get),
    components(schemas(User)),
    servers((url = "/user")),
    modifiers(&UpdatePaths)
)]
pub struct ApiAuthDoc;

#[utoipa::path(get, responses((status = 200, body = User)))]
#[get("/")]
async fn user_get(user: User) -> Response<User> {
    Ok(Json(user))
}

pub fn router() -> Scope {
    Scope::new("/user").service(user_get)
    // .service(user_get)
    // .service(user_update)
    // .service(user_update_photo)
    // .service(user_delete_photo)
    // .service(user_wallet_test)
    // .service(user_transactions_list)
}
