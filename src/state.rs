use limbo::Connection;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter; // Import Limbo types

pub struct AppState {
    pub db_conn: Connection,
}

pub type SharedState = Arc<RwLock<AppState>>;

pub fn with_state(
    state: SharedState,
) -> impl Filter<Extract = (SharedState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
