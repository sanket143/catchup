use crate::models::User;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

pub struct AppState {
    pub db_pool: SqlitePool,
    pub user: Option<User>,
}

pub async fn create_app_state(database_url: &str) -> Result<AppState, sqlx::Error> {
    let db_pool = SqlitePool::connect(database_url).await?;

    Ok(AppState {
        user: None,
        db_pool,
    })
}

pub fn with_state(
    state: Arc<RwLock<AppState>>,
) -> impl Filter<Extract = (Arc<RwLock<AppState>>,), Error = warp::Rejection> + Clone {
    warp::cookie::optional("username").and_then(move |username: Option<String>| {
        let state = state.clone();

        async move {
            if let Some(username) = username {
                let mut locked = state.write().await;
                locked.user = Some(User::new(username));
            }

            Ok::<_, warp::Rejection>(state)
        }
    })
}
