use anyhow::Result;
use handlers::{api, index};
use state::{create_app_state, with_state};
use warp::Filter;

mod db;
mod handlers;
mod models;
mod state;
mod templates;

pub async fn run() -> Result<()> {
    templates::init::init_tera("templates/**/*.html").expect("Failed to initialize Tera");

    let db_url = dotenvy::var("DATABASE_URL")?;
    let app_state = create_app_state(&db_url).await?;

    let index = warp::path::end()
        .and(warp::get())
        .and(with_state(app_state.clone()))
        .and_then(index::handler);

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let static_files = warp::path("static").and(warp::fs::dir("static"));

    let routes = index
        .or(hello)
        .or(static_files)
        .or(api::handle_routes(app_state.clone()));

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
