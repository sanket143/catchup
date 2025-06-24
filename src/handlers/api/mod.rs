use warp::Filter;
use warp::{reject::Rejection, reply::Reply};

use crate::state::{AppState, with_state};

mod sync_problems;

pub fn handle_routes(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let sync_problems_api = warp::path!("api" / "sync-problems")
        .and(warp::post())
        .and(with_state(state.clone()))
        .and_then(sync_problems::handler);

    sync_problems_api
}
