use std::collections::HashMap;

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


type Prices = HashMap<String, i64>;
static mut PRICES: Option<HashMap<Prices> = None;
static mut PRICES_UPDATE: i64 = 0;

async fn prices() -> Prices {
    if PRICES.is_none() || PRICES_UPDATE < now() - 600 {}
}


#[derive(OpenApi)]
#[openapi(
    tags((name = "api::vnum")),
    paths(prices),
    components(),
    servers((url = "/vnum")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(get, responses((status = 200, body = User)))]
#[get("/prices/")]
async fn prices_get(user: User) -> String {
    "hi".into()
}

pub fn router() -> Scope {
    Scope::new("/vnum").service(prices)
    // .service(user_get)
    // .service(user_update)
    // .service(user_update_photo)
    // .service(user_delete_photo)
    // .service(user_wallet_test)
    // .service(user_transactions_list)
}
