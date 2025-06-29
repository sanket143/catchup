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

    // contest api routes
    let create_contest = api(String::from("contest"))
        .and(warp::path("create"))
        .and(warp::body::json())
        .and_then(contest::create);
    let contest_count = api(String::from("contest"))
        .and(warp::path("count"))
        .and_then(contest::count);
    let contest_current = api(String::from("contest"))
        .and(warp::path("current"))
        .and_then(contest::current);
    let contest_evaluate = api(String::from("contest"))
        .and(warp::path("evaluate"))
        .and(warp::body::json())
        .and_then(contest::evaluate);

    let routes = sync_problems_api
        .or(create_contest)
        .or(contest_count)
        .or(contest_current)
        .or(contest_evaluate);

    routes
}
