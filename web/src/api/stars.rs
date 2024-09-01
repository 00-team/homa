use std::collections::HashMap;

use actix_web::web::{Data, Json};
use actix_web::{get, Scope};
use utoipa::OpenApi;

use crate::config::Config;
use crate::docs::UpdatePaths;
use crate::general::general_get;
use crate::models::user::User;
use crate::models::Response;
use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::stars")),
    paths(price),
    components(schemas()),
    servers((url = "/stars")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(get, responses((status = 200)))]
/// Star Price
#[get("/price/")]
async fn price(_: User, state: Data<AppState>) -> Response<f64> {
    let general = general_get(&state.sql).await?;
    let tax = 1.0 + general.star_tax as f64 / 100.0;
    let price = Config::STAR_COST * tax * general.usd_irr as f64;

    Ok(Json(price))
}

pub fn router() -> Scope {
    Scope::new("/stars").service(price)
}
