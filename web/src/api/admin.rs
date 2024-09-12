use actix_web::web::{Data, Json};
use actix_web::{get, patch, HttpResponse, Scope};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::docs::UpdatePaths;
use crate::general::{general_get, general_set, General};

use crate::models::user::Admin;
use crate::models::{AppErr, Response};
use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::admin")),
    paths(
        get_general, update_general
    ),
    components(schemas(General, UpdateGeneralBody)),
    servers((url = "/admin")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(get, responses((status = 200, body = General)))]
/// Get General
#[get("/general/")]
async fn get_general(_: Admin, state: Data<AppState>) -> Response<General> {
    let general = general_get(&state.sql).await?;
    Ok(Json(general))
}

#[derive(Deserialize, ToSchema)]
struct UpdateGeneralBody {
    rub_irr: i64,
    usd_irr: i64,
    star_tax: i64,
    phone_tax: i64,
    disable_wallet: bool,
    disable_stars: bool,
    disable_phone: bool,
}

#[utoipa::path(
    patch,
    request_body = UpdateGeneralBody,
    responses((status = 200))
)]
/// Update General
#[patch("/general/")]
async fn update_general(
    _: Admin, body: Json<UpdateGeneralBody>, state: Data<AppState>,
) -> Result<HttpResponse, AppErr> {
    let mut general = general_get(&state.sql).await?;

    general.rub_irr = body.rub_irr;
    general.usd_irr = body.usd_irr;
    general.star_tax = body.star_tax;
    general.phone_tax = body.phone_tax;
    general.disable_wallet = body.disable_wallet;
    general.disable_stars = body.disable_stars;
    general.disable_phone = body.disable_phone;

    general_set(&state.sql, &general).await?;

    Ok(HttpResponse::Ok().finish())
}

pub fn router() -> Scope {
    Scope::new("/admin").service(get_general).service(update_general)
}
