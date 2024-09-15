use crate::docs::UpdatePaths;
use crate::models::order::{OrderStatus, StarOrder};
use crate::models::user::Admin;
use crate::models::{AppErrBadRequest, Response};
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
        "select * from star_orders where status = ? order by id desc limit 32 offset ?",
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
            order.status = OrderStatus::Done;
            order.hash = Some(hash);
        }
        StarOrderUpdateBody::Refunded => {
            order.status = OrderStatus::Refunded;
        }
    }

    sqlx::query! {
        "update star_orders set status = ?, hash = ? where id = ?",
        order.status, order.hash, order.id
    }
    .execute(&state.sql)
    .await?;

    Ok(Json(order))
}

pub fn router() -> Scope {
    Scope::new("/stars").service(list).service(update)
}
