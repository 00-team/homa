use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use actix_web::web::{Json, Query};
use actix_web::{get, HttpResponse, Scope};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::Value;
use utoipa::{IntoParams, OpenApi, ToSchema};

use crate::docs::UpdatePaths;
use crate::models::{AppErr, Response, User};
use crate::vendor;

lazy_static! {
    static ref PRICES: Mutex<HashMap<String, i64>> = Mutex::new(HashMap::new());
    static ref PRICES_UPDATE: i64 = 0;
}

// type Prices = HashMap<String, i64>;
// static mut PRICES: Option<Prices> = None;
// static mut PRICES_UPDATE: i64 = 0;
//
// async fn prices() -> Prices {
//     if PRICES.is_none() || PRICES_UPDATE < now() - 600 {}
//
//     if let Some(p) = PRICES {}
// }

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::vendor")),
    paths(prices_get, check_service),
    components(),
    servers((url = "/vendor")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(get, responses((status = 200, body = User)))]
#[get("/prices/")]
async fn prices_get(_: User) -> Response<HashMap<String, i64>> {
    let mut x = PRICES.lock().unwrap();
    let len = x.len();
    x.insert(format!("hi-{}", len), 12);
    Ok(Json(x.deref().clone()))
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
struct CheckServiceQuery {
    service: String,
}

#[utoipa::path(
    get,
    params(CheckServiceQuery),
    responses((status = 200))
)]
#[get("/check-service/")]
async fn check_service(
    _: User, q: Query<CheckServiceQuery>,
) -> Response<Value> {
    let args = vec![("service", q.service.as_str())];
    let result = vendor::request("getPricesVerification", args).await?;
    return Ok(Json(result));
}

pub fn router() -> Scope {
    Scope::new("/vendor").service(prices_get).service(check_service)
    // .service(user_get)
    // .service(user_update)
    // .service(user_update_photo)
    // .service(user_delete_photo)
    // .service(user_wallet_test)
    // .service(user_transactions_list)
}
