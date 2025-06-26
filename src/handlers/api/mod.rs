use crate::state::{AppState, with_state};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;
use warp::{reject::Rejection, reply::Reply};

mod sync_problems;

pub fn handle_routes(
    state: Arc<RwLock<AppState>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let sync_problems_api = warp::path!("api" / "sync-problems")
        .and(warp::post())
        .and(with_state(state))
        .and_then(sync_problems::handler);

    sync_problems_api
}
