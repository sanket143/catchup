use anyhow::Result;
use handlers::api;
use state::{create_app_state, with_state};
use std::sync::Arc;
use tokio::sync::RwLock;

mod db;
mod handlers;
mod macros;
mod models;
mod state;

pub async fn run() -> Result<()> {
    let db_file = dotenvy::var("DATABASE_FILE")?;
    let app_state = Arc::new(RwLock::new(create_app_state(&db_file).await?));

    let routes = api::handle_routes(app_state.clone());

    warp::serve(routes).run(([127, 0, 0, 1], 3001)).await;

    Ok(())
}
