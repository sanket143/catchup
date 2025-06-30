use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use ketchup::{db::get_db_pool, handlers::register};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let pool = Arc::new(
        get_db_pool()
            .await
            .expect("Failed to create db pool for Sqlite"),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(register)
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
