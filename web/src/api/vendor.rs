use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use actix_web::web::{Json, Path};
use actix_web::{get, post, HttpResponse, Scope};
use lazy_static::lazy_static;
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::config::config;
use crate::docs::UpdatePaths;
use crate::models::{AppErr, AppErrForbidden, Response, User};
use crate::utils;
use crate::vendor;

type Prices = HashMap<String, (i64, i64)>;
lazy_static! {
    static ref PRICES: Mutex<Prices> = Mutex::new(HashMap::new());
    static ref PRICES_UPDATE: Mutex<i64> = Mutex::new(0);
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
    paths(prices_get, check_service, sms_callback),
    components(schemas(SmsData)),
    servers((url = "/vendor")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(get, responses((status = 200, body = User)))]
#[get("/prices/")]
async fn prices_get(_: User) -> Response<Prices> {
    let x = PRICES.lock().unwrap();
    Ok(Json(x.deref().clone()))
}

#[utoipa::path(get, responses((status = 200)))]
#[get("/check-service/")]
async fn check_service(_: User) -> Response<Prices> {
    let now = utils::now();
    let mut update = PRICES_UPDATE.lock().unwrap();
    let mut prices = PRICES.lock().unwrap();
    if *update + 600 < now {
        let result = vendor::request("getPrices", vec![]).await?;
        prices.clear();
        *update = now;

        let result = if let Some(v) = result.as_object() {
            v
        } else {
            return Err(AppErr::default());
        };

        result.iter().for_each(|(country, v)| {
            v.as_object().unwrap().iter().for_each(|(service, vv)| {
                let vv = vv.as_object().unwrap();
                let count = vv.get("count").unwrap().as_i64().unwrap();
                let cost = vv.get("cost").unwrap().as_i64().unwrap();

                if count == 0 {
                    return;
                }

                prices.insert(format!("{country}-{service}"), (cost, count));
            });
        });
    }

    Ok(Json(prices.deref().clone()))
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct SmsData {
    activation_id: i64,
    service: String,
    text: String,
    code: String,
    country: i64,
    received_at: String,
}

#[utoipa::path(
    post,
    params(("pass" = String, Path,)),
    request_body = SmsData,
    responses((status = 200))
)]
#[post("/sms-callback/{pass}/")]
async fn sms_callback(
    data: Json<SmsData>, path: Path<(String,)>,
) -> Result<HttpResponse, AppErr> {
    if path.0 != config().sms_cb_pass {
        return Err(AppErrForbidden("invalid pass"));
    }

    utils::send_webhook(
        "Sms",
        &format!(
            "
id: {}
service: {}
text: `{}`
code: `{}`
country: {}
receivedAt: {}
",
            data.activation_id,
            data.service,
            data.text,
            data.code,
            data.country,
            data.received_at
        ),
        13868854,
    )
    .await;
    Ok(HttpResponse::Ok().body(""))
}

pub fn router() -> Scope {
    Scope::new("/vendor")
        .service(prices_get)
        .service(check_service)
        .service(sms_callback)
}
