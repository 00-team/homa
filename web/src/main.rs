use std::{env, os::unix::fs::PermissionsExt};

// use actix_files as af;
use actix_web::{
    get,
    // http::header::ContentType,
    middleware,
    web::Data,
    App,
    HttpResponse,
    HttpServer,
    Responder,
};
use sqlx::{Pool, Sqlite, SqlitePool};

pub struct AppState {
    pub sql: Pool<Sqlite>,
}

#[get("/")]
async fn index() -> impl Responder {
    // let result = read_to_string("dist/index.html")
    //     .unwrap_or("err reading index.html".to_string());
    // HttpResponse::Ok().content_type(ContentType::html()).body(result)
    HttpResponse::Ok().body("hi from thora index")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::from_path("../.secrets.env").expect("could not read secrets.env");
    pretty_env_logger::init();

    let pool =
        SqlitePool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL was not found in env"))
            .await
            .expect("sqlite pool initialization failed");

    sqlx::migrate!().run(&pool).await.expect("migration failed");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("%s %r %Ts"))
            .app_data(Data::new(AppState { sql: pool.clone() }))
            .service(index)
    });

    let server = if cfg!(debug_assertions) {
        server.bind(("127.0.0.1", 7200)).unwrap()
    } else {
        const PATH: &'static str = "/usr/share/nginx/sockets/thora.web.sock";
        let s = server.bind_uds(PATH).unwrap();
        std::fs::set_permissions(PATH, std::fs::Permissions::from_mode(0o777))?;
        s
    };

    server.run().await
}
