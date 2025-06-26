use anyhow::Result;
use handlers::{api, index, local_contest, login, settings};
use state::{create_app_state, with_state};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

mod db;
mod handlers;
mod models;
mod state;
mod templates;

pub async fn run() -> Result<()> {
    templates::init::init_tera("templates/**/*.html").expect("Failed to initialize Tera");

    let db_url = dotenvy::var("DATABASE_URL")?;
    let app_state = Arc::new(RwLock::new(create_app_state(&db_url).await?));

    let index = warp::path::end()
        .and(warp::get())
        .and(with_state(app_state.clone()))
        .and_then(index::handler);
    let local_contest = warp::path("local-contest")
        .and(warp::get())
        .and(with_state(app_state.clone()))
        .and_then(local_contest::handler);
    let login = warp::path("login")
        .and(warp::get())
        .and(with_state(app_state.clone()))
        .and_then(login::handler);
    let settings = warp::path("settings")
        .and(warp::get())
        .and(with_state(app_state.clone()))
        .and_then(settings::handler);

    let static_files = warp::path("static").and(warp::fs::dir("static"));

    let routes = index
        .or(local_contest)
        .or(login)
        .or(settings)
        .or(static_files)
        .or(api::handle_routes(app_state.clone()));

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
