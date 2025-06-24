// src/state.rs (for sqlx)
use sqlx::SqlitePool;
use warp::Filter;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
}

pub async fn create_app_state(database_url: &str) -> Result<AppState, sqlx::Error> {
    let db_pool = SqlitePool::connect(database_url).await?;

    Ok(AppState { db_pool })
}

pub fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
