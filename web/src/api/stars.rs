use crate::config::Config;
use crate::docs::UpdatePaths;
use crate::models::order::StarOrder;
use crate::models::user::User;
use crate::models::{AppErr, AppErrBadRequest, Response};
use crate::{utils, AppState};
use actix_web::web::{Data, Json};
use actix_web::{get, post, Scope};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::stars")),
    paths(star_price, buy),
    components(schemas(StarBuyBody)),
    servers((url = "/stars")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(get, responses((status = 200, body = f64)))]
/// Star Price
#[get("/price/")]
async fn star_price(_: User, state: Data<AppState>) -> Response<f64> {
    let general = state.general.lock()?;
    let tax = 1.0 + general.star_tax as f64 / 100.0;
    let price = Config::STAR_COST * tax * general.usd_irr as f64;

    Ok(Json(price))
}

#[derive(Deserialize, ToSchema)]
struct StarBuyBody {
    amount: i64,
}

#[utoipa::path(post, responses((status = 200, body = StarOrder)))]
/// Buy
#[post("/buy/")]
async fn buy(
    user: User, body: Json<StarBuyBody>, state: Data<AppState>,
) -> Response<StarOrder> {
    let general = state.general.lock()?;
    if general.disable_stars {
        return Err(AppErrBadRequest("خرید استار درحال حاظر دردسترس نمی باشد"));
    }

    let tax = 1.0 + general.star_tax as f64 / 100.0;
    let price = Config::STAR_COST * tax * general.usd_irr as f64;
    let cost = price as i64 * body.amount;
    if cost > user.wallet {
        return Err(AppErr::too_poor());
    }
    let now = utils::now();

    let mut order = StarOrder {
        user: user.id,
        amount: body.amount,
        timestamp: now,
        cost,
        ..Default::default()
    };

    let result = sqlx::query! {
        "insert into star_orders(user, amount, cost, timestamp) values(?,?,?,?)",
        order.user, order.amount, order.cost, order.timestamp,
    }
    .execute(&state.sql)
    .await?;

    order.id = result.last_insert_rowid();

    sqlx::query! {
        "update users set wallet = wallet - ? where id = ?",
        cost, user.id
    }
    .execute(&state.sql)
    .await?;

    Ok(Json(order))
}

pub fn router() -> Scope {
    Scope::new("/stars").service(star_price).service(buy)
}
