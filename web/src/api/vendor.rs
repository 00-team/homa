use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpResponse, Scope};
use lazy_static::lazy_static;
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::config::{config, Config};
use crate::docs::UpdatePaths;
use crate::general::{general_get, general_set};
use crate::models::user::User;
use crate::models::{AppErr, AppErrForbidden, Response};
use crate::vendor::{self, rub_irr_price};
use crate::{utils, AppState};

type Prices = HashMap<String, (f64, i64)>;
lazy_static! {
    static ref PRICES: Mutex<Prices> = Mutex::new(HashMap::new());
    static ref PRICES_UPDATE: Mutex<i64> = Mutex::new(0);
}

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::vendor")),
    paths(prices, sms_callback),
    components(schemas(SmsData)),
    servers((url = "/vendor")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(get, responses((status = 200)))]
#[get("/prices/")]
async fn prices(_: User, state: Data<AppState>) -> Response<Prices> {
    let now = utils::now();
    let mut update = PRICES_UPDATE.lock().expect("PRICES_UPDATE lock err");
    let mut prices = PRICES.lock().expect("PRICES lock err");
    if *update + 600 < now {
        *update = now;

        let result = vendor::request("getPrices", vec![]).await?;
        prices.clear();

        let mut general = general_get(&state.sql).await?;
        if general.rub_irr_update + 86400 < now {
            general.rub_irr_update = now;
            general.rub_irr = rub_irr_price().await?;
            general_set(&state.sql, &general).await?;
        }

        result.as_object().expect("result is not an object").iter().for_each(
            |(country, v)| {
                v.as_object().expect("invalid response L1").iter().for_each(
                    |(service, vv)| {
                        let vv = vv.as_object().expect("invalid response L2");
                        let count = vv.get("count").expect("count not found");
                        let count = count.as_i64().expect("count is NaN");
                        let cost = vv.get("cost").expect("cost not found");
                        let cost = cost.as_f64().expect("cost is NaN");

                        if count == 0 {
                            return;
                        }

                        let price = cost * general.rub_irr as f64 * Config::TAX;
                        let price = ((price / 1e4).ceil() * 1e4).max(15e4);

                        prices.insert(
                            format!("{country}-{service}"),
                            (price, count),
                        );
                    },
                );
            },
        );
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
    Scope::new("/vendor").service(prices).service(sms_callback)
}
