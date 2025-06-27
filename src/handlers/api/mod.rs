use crate::state::{AppState, with_state};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;
use warp::{reject::Rejection, reply::Reply};

mod contest;
mod sync_problems;

pub fn handle_routes(
    state: Arc<RwLock<AppState>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let api = |s: String| {
        warp::path("api").and(
            warp::path(s)
                .and(warp::post())
                .and(with_state(state.clone())),
        )
    };

    let sync_problems_api = api(String::from("sync-problems")).and_then(sync_problems::handler);
    let list_contest_problems = api(String::from("contest"))
        .and(warp::path("problems"))
        .and(warp::body::json())
        .and_then(contest::list_problems);

    sync_problems_api.or(list_contest_problems)
}
