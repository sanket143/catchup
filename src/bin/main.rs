use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use catchup::{db::get_db_pool, handlers::register, schemas::root::create_schema};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let pool = Arc::new(
        get_db_pool()
            .await
            .expect("Failed to create db pool for Sqlite"),
    );

    println!("Running server on :3001");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(create_schema()))
            .configure(register)
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}
