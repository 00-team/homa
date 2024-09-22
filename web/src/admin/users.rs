use crate::config::config;
use crate::docs::UpdatePaths;
use crate::models::user::{Admin, User};
use crate::models::{ListInput, Response};
use crate::AppState;
use actix_web::web::{Data, Json, Query};
use actix_web::{get, Scope};
use serde::Deserialize;
use serde_json::Value;
use utoipa::{IntoParams, OpenApi};

#[derive(OpenApi)]
#[openapi(
    tags((name = "admin::users")),
    paths(list, username),
    components(schemas()),
    servers((url = "/users")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    params(ListInput),
    responses((status = 200, body = Vec<User>))
)]
/// List Of Users
#[get("/")]
async fn list(
    _: Admin, q: Query<ListInput>, state: Data<AppState>,
) -> Response<Vec<User>> {
    let offset = q.page * 32;
    let result = sqlx::query_as! {
        User,
        "select * from users order by id desc limit 32 offset ?",
        offset
    }
    .fetch_all(&state.sql)
    .await?;

    Ok(Json(result))
}

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

    if cfg!(debug_assertions) {
        return Ok(Json(user.username));
    }

    let client = awc::Client::new();
    let url = format!(
        "https://api.telegram.org/bot{}/getChat?chat_id={}",
        config().bot_token,
        user.id
    );
    let request = client.post(&url);

    let username = request
        .send()
        .await?
        .json::<Value>()
        .await?
        .as_object()
        .and_then(|v| v.get("result").and_then(|v| v.as_object()))
        .and_then(|v| v.get("username").and_then(|v| v.as_str()))
        .and_then(|v| Some(v.to_string()));

    if username != user.username {
        sqlx::query! {
            "update users set username = ? where id = ?",
            username, user.id
        }
        .execute(&state.sql)
        .await?;
    }

    Ok(Json(username))
}

pub fn router() -> Scope {
    Scope::new("/users").service(username).service(list)
}
