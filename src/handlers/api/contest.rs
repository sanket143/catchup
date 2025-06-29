use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    models::{
        contest::{add_problem_in_contest, contest_count, create_contest, current_contest},
        contest_problem_level::get_problem_level_details,
        user::create_user,
    },
    state::AppState,
    warp_err,
};

#[derive(Deserialize, Debug)]
pub struct CreateContestInput {
    name: String,
}

pub async fn create(
    state: Arc<RwLock<AppState>>,
    input: CreateContestInput,
) -> Result<impl Reply, Rejection> {
    let user = (&state.read().await)
        .user
        .clone()
        .expect("User not logged in");

    let mut tx = warp_err!(state.read().await.db_pool.begin().await);

    // TODO: If there exists a current running contest, don't create a new contest,
    // rather throw an error saying there's a contest already running

    let user = warp_err!(create_user(&mut *tx, &user.get_username()).await);
    let contest_requirement = warp_err!(get_problem_level_details(&mut *tx, &user.level).await);

    let contest = warp_err!(
        create_contest(
            &mut *tx,
            &input.name,
            &contest_requirement.duration,
            &user.username
        )
        .await
    );

    let problem_tag = "implementation";

    for rating in [
        contest_requirement.problem_rating_level_1,
        contest_requirement.problem_rating_level_2,
        contest_requirement.problem_rating_level_3,
        contest_requirement.problem_rating_level_4,
    ]
    .iter()
    {
        warp_err!(add_problem_in_contest(&mut *tx, &contest.id, rating, problem_tag).await);
    }

    warp_err!(tx.commit().await);

    Ok(warp::reply::json(&serde_json::json!({
        "id": contest.id
    })))
}

pub async fn count(state: Arc<RwLock<AppState>>) -> Result<impl Reply, Rejection> {
    let state = &state.read().await;
    let mut tx = warp_err!(state.db_pool.begin().await);
    let username = state.user.clone().unwrap().get_username();

    let count = warp_err!(contest_count(&mut *tx, &username).await);

    Ok(warp::reply::json(&serde_json::json!({
        "count": count
    })))
}

pub async fn current(state: Arc<RwLock<AppState>>) -> Result<impl Reply, Rejection> {
    let state = &state.read().await;
    let mut tx = warp_err!(state.db_pool.begin().await);
    let username = state.user.clone().unwrap().get_username();

    let contest = warp_err!(current_contest(&mut *tx, &username).await);

    if let Some(contest) = contest {
        let problems = warp_err!(contest.get_problems(&mut *tx).await);
        warp_err!(tx.commit().await);

        return Ok(warp::reply::json(&serde_json::json!({
            "contest": contest,
            "problems": problems
        })));
    }

    Ok(warp::reply::json(&serde_json::json!({
        "contest": contest
    })))
}
