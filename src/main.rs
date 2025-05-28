use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use connections::postgres::Db;
use dotenv::dotenv;
use handler::{create_user, get_all_users};
use std::io;

use config::Config;

mod config;
mod connections;
mod handler;
mod logs;

async fn healthchecker() -> impl Responder {
    const MESSAGE: &str = "Complete RUST API";

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": MESSAGE
    }))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    logs::init_standard_logger();

    let config = Config::init();

    // init database pool
    let db = Db::new(&config.database_url)
        .await
        .expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default().log_target("@ => :"))
            .app_data(web::Data::new(db.clone()))
            .service(
                web::scope("/api")
                    .service(web::resource("/healthchecker").route(web::get().to(healthchecker)))
                    .service(
                        web::resource("/user")
                            .route(web::post().to(create_user))
                            .route(web::get().to(get_all_users)),
                    ),
            )
    })
    .bind(("127.0.0.1", config.port))?
    .workers(2)
    .run()
    .await
}
