use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use actix_web::web::{Json, Path, Query};
use actix_web::{get, post, HttpResponse, Scope};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::Value;
use utoipa::{IntoParams, OpenApi, ToSchema};

use crate::config::config;
use crate::docs::UpdatePaths;
use crate::models::{AppErr, AppErrForbidden, Response, User};
use crate::utils::send_webhook;
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
    paths(prices_get, check_service, sms_callback),
    components(schemas(SmsData)),
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
    time: u16,
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
    if q.time == 20 {
        let args = vec![("service", q.service.as_str())];
        let result = vendor::request("getPricesVerification", args).await?;
        Ok(Json(result))
    } else {
        let time = q.time.to_string();
        let args = vec![("rent_time", time.as_str())];
        let result =
            vendor::request("getRentServicesAndCountries", args).await?;
        Ok(Json(result))
    }
}

#[derive(Deserialize, ToSchema)]
// #[allow(non_snake_case)]
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

    send_webhook(
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
