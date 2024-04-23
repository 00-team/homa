use actix_web::web::Json;
use actix_web::{get, Scope};
use utoipa::OpenApi;

use crate::docs::UpdatePaths;
use crate::models::{Response, User};

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::user")),
    paths(user_get),
    components(schemas(User)),
    servers((url = "/user")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(get, responses((status = 200, body = User)))]
#[get("/")]
async fn user_get(user: User) -> Response<User> {
    Ok(Json(user))
}

pub fn router() -> Scope {
    Scope::new("/user").service(user_get)
    // .service(user_get)
    // .service(user_update)
    // .service(user_update_photo)
    // .service(user_delete_photo)
    // .service(user_wallet_test)
    // .service(user_transactions_list)
}
