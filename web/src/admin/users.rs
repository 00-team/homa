use crate::docs::UpdatePaths;
use crate::models::user::{Admin, User};
use crate::models::Response;
use crate::AppState;
use actix_web::web::{Data, Json, Query};
use actix_web::{get, Scope};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};

#[derive(OpenApi)]
#[openapi(
    tags((name = "admin::users")),
    paths(username),
    components(schemas()),
    servers((url = "/users")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[derive(Deserialize, IntoParams)]
struct UsernameQuery {
    #[param(example = 12)]
    id: i64,
}

#[utoipa::path(
    get,
    params(UsernameQuery),
    responses((status = 200, body = Option<String>))
)]
/// Username
#[get("/username/")]
async fn username(
    _: Admin, q: Query<UsernameQuery>, state: Data<AppState>,
) -> Response<Option<String>> {
    let user = sqlx::query_as! {
        User,
        "select * from users where id = ?",
        q.id
    }
    .fetch_one(&state.sql)
    .await?;

    Ok(Json(user.username))
}

pub fn router() -> Scope {
    Scope::new("/users").service(username)
}
