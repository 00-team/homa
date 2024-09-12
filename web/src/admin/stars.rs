use crate::docs::UpdatePaths;
use crate::models::order::{OrderStatus, StarOrder};
use crate::models::user::Admin;
use crate::models::Response;
use crate::AppState;
use actix_web::web::{Data, Json, Query};
use actix_web::{get, Scope};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};

#[derive(OpenApi)]
#[openapi(
    tags((name = "admin::stars")),
    paths(list),
    components(schemas()),
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

pub fn router() -> Scope {
    Scope::new("/stars").service(list)
}
