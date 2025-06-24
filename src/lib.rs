use anyhow::Result;
use handlers::{api, index};
use limbo::{Builder, params_from_iter};
use state::{AppState, with_state};
use std::fs;
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
    let db_path = "data/data.db";
    let db_conn = Builder::new_local(db_path).build().await?.connect()?;
    let app_state = Arc::new(RwLock::new(AppState { db_conn }));

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
    let db_path = "data/data.db";
    let db_conn = Builder::new_local(db_path).build().await?.connect()?;

    let mut migrations = fs::read_dir("./migrations")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?;

    migrations.sort();

    for path in migrations.iter() {
        println!("{}", path.display());
        let query = fs::read_to_string(path).unwrap();

        // lazy spliting, if there's ';' in comment then this will break;
        for sql in query.split(';') {
            if sql.trim().len() > 0 {
                db_conn.execute(&sql, ()).await?;
            }
        }
    }

    Ok(())
}
