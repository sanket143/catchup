use anyhow::Result;
use handlers::{api, index};
use state::{create_app_state, with_state};
use std::fs;
use warp::Filter;

mod db;
mod handlers;
mod models;
mod state;
mod templates;

pub async fn run() -> Result<()> {
    templates::init::init_tera("templates/**/*.html").expect("Failed to initialize Tera");
    let db_url = "sqlite:data/data.db";
    let app_state = create_app_state(db_url).await?;

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

pub async fn migrate() -> Result<()> {
    let db_url = "sqlite:data/data.db";
    let app_state = create_app_state(db_url).await?;

    let mut migrations = fs::read_dir("./migrations")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;

    migrations.sort();

    for path in migrations.iter() {
        // TODO: Use sqlx::migrate
        let query = fs::read_to_string(path).unwrap();
        sqlx::query(&query).execute(&app_state.db_pool).await?;
    }

    Ok(())
}
