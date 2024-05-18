use std::collections::HashMap;

use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpResponse, Scope};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::config::{config, Config};
use crate::docs::UpdatePaths;
use crate::general::{general_get, general_set, PriceValue};
use crate::models::user::User;
use crate::models::{AppErr, AppErrForbidden, Response};
use crate::vendor::{self, rub_irr_price};
use crate::{utils, AppState};

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::vendor")),
    paths(prices, sms_callback),
    components(schemas(SmsData)),
    servers((url = "/vendor")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

type Prices = HashMap<String, i64>;

#[utoipa::path(get, responses((status = 200)))]
#[get("/prices/")]
async fn prices(_: User, state: Data<AppState>) -> Response<Prices> {
    let now = utils::now();
    let mut general = general_get(&state.sql).await?;
    let mut update_general = false;

    if general.rub_irr_update + 86400 < now {
        update_general = true;
        general.rub_irr_update = now;
        general.rub_irr = rub_irr_price().await?;
    }

    if general.prices_update + 600 < now {
        update_general = true;
        general.prices_update = now;

        let avg_diff =
            if general.price_diff_count != 0 && general.price_diff_total != 0 {
                (general.price_diff_total / general.price_diff_count) as f64
            } else {
                10.0
            };

        let result = vendor::request("getPrices", vec![]).await?;

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

                        let key = format!("{country}-{service}");
                        if let Some(p) = general.prices.get_mut(&key) {
                            p.count = count;
                            p.cost_api = cost + avg_diff;
                        } else {
                            general.prices.insert(
                                key,
                                PriceValue {
                                    cost_api: cost,
                                    count,
                                    ..Default::default()
                                },
                            );
                        }
                    },
                );
            },
        );
    }

    if update_general {
        general_set(&state.sql, &general).await?;
    }

    let prices: Prices = general
        .prices
        .iter()
        .map(|(k, v)| {
            let cost = if v.cost_buy > 0.0 && v.timestamp + 30 * 86400 > now {
                v.cost_buy
            } else {
                v.cost_api
            };

            let p = cost * general.rub_irr as f64 * Config::TAX;
            let p = ((p / 1e4).ceil() * 1e4).max(15e4) as i64;
            (k.clone(), p)
        })
        .collect();

    Ok(Json(prices))
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
