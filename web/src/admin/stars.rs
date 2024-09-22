use crate::config::Config;
use crate::docs::UpdatePaths;
use crate::general::general_set;
use crate::models::order::{OrderStatus, StarOrder};
use crate::models::user::{Admin, User};
use crate::models::{AppErrBadRequest, Response};
use crate::utils::send_star_order;
use crate::AppState;
use actix_web::web::{Data, Json, Query};
use actix_web::{get, patch, Scope};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    tags((name = "admin::stars")),
    paths(list, update),
    components(schemas(StarOrderUpdateBody)),
    servers((url = "/stars")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[derive(Deserialize, IntoParams)]
struct ListQuery {
    #[param(example = 0)]
    page: u32,
    status: OrderStatus,
}

#[utoipa::path(
    get,
    params(ListQuery),
    responses((status = 200, body = Vec<StarOrder>))
)]
/// List Of Orders
#[get("/")]
async fn list(
    _: Admin, q: Query<ListQuery>, state: Data<AppState>,
) -> Response<Vec<StarOrder>> {
    let offset = q.page * 32;
    let result = sqlx::query_as! {
        StarOrder,
        "select * from star_orders where status = ? order by id asc limit 32 offset ?",
        q.status, offset
    }
    .fetch_all(&state.sql)
    .await?;

    Ok(Json(result))
}

#[derive(Deserialize, ToSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
enum StarOrderUpdateBody {
    Refunded,
    Done { hash: String },
}

#[utoipa::path(
    patch,
    params(("soid" = i64, Path,)),
    responses((status = 200, body = StarOrder))
)]
/// Update Order
#[patch("/{soid}/")]
async fn update(
    _: Admin, order: StarOrder, body: Json<StarOrderUpdateBody>,
    state: Data<AppState>,
) -> Response<StarOrder> {
    let mut order = order;
    if !matches!(order.status, OrderStatus::Wating) {
        return Err(AppErrBadRequest("order is already finished"));
    }

    match body.0 {
        StarOrderUpdateBody::Done { hash } => {
            let mut general = state.general.lock()?;
            general.money_total += order.cost;

            let raw = order.amount as f64
                * Config::STAR_COST
                * general.usd_irr as f64;

            general.money_gain += order.cost - raw as i64;
            general_set(&state.sql, &general).await?;

            order.status = OrderStatus::Done;
            order.hash = Some(hash);
        }
        StarOrderUpdateBody::Refunded => {
            sqlx::query! {
                "update users set wallet = wallet + ? where id = ?",
                order.cost, order.user
            }
            .execute(&state.sql)
            .await?;

            order.status = OrderStatus::Refunded;
        }
    }

    sqlx::query! {
        "update star_orders set status = ?, hash = ? where id = ?",
        order.status, order.hash, order.id
    }
    .execute(&state.sql)
    .await?;

    let user = sqlx::query_as! {
        User,
        "select * from users where id = ?",
        order.user
    }
    .fetch_one(&state.sql)
    .await?;

    send_star_order(&user, &order).await;

    Ok(Json(order))
}

pub fn router() -> Scope {
    Scope::new("/stars").service(list).service(update)
}
