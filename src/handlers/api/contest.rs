use serde::Deserialize;
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    models::{
        codeforces::Submission,
        contest::{
            add_problem_in_contest, add_problem_in_contest_by_id, contest_count, create_contest,
            current_contest, get_contest_id, update_contest,
        },
        contest_problem_level::get_problem_level_details,
        contest_problem_map::update_contest_problem_map,
        problem_tag_group::get_random_problem_tag_group,
        user::create_user,
    },
    state::AppState,
    warp_err,
};

#[derive(Deserialize, Debug)]
pub struct CreateContestInput {
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct EvaluateContestInput {
    #[serde(rename = "contestId")]
    contest_id: i64,
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
    let problem_tag_group = warp_err!(get_random_problem_tag_group(&mut *tx).await);

    let contest = warp_err!(
        create_contest(
            &mut *tx,
            &input.name,
            &contest_requirement.duration,
            &user.username,
            &problem_tag_group,
        )
        .await
    );

    for rating in [
        contest_requirement.problem_rating_level_1,
        contest_requirement.problem_rating_level_2,
        contest_requirement.problem_rating_level_3,
        contest_requirement.problem_rating_level_4,
    ]
    .iter()
    {
        // warp_err!(add_problem_in_contest(&mut *tx, &contest.id, rating, &problem_tag_group).await);
    }

    for uid in ["CF/2117/B", "CF/2117/A", "CF/2112/C", "CF/2112/B"].iter() {
        warp_err!(add_problem_in_contest_by_id(&mut *tx, &contest.id, uid).await);
    }

    warp_err!(tx.commit().await);

    println!("{:?}", contest);

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

#[derive(Deserialize)]
pub struct Response {
    pub result: Vec<Submission>,
}

pub async fn evaluate(
    state: Arc<RwLock<AppState>>,
    input: EvaluateContestInput,
) -> Result<impl Reply, Rejection> {
    println!("Evaluating contest: {}", input.contest_id);
    let state = &state.read().await;
    let mut tx = warp_err!(state.db_pool.begin().await);

    let contest = warp_err!(get_contest_id(&mut *tx, &input.contest_id).await);
    let problems = warp_err!(contest.get_problems(&mut *tx).await);

    let body: Response = warp_err!(
        warp_err!(
            ureq::get("https://codeforces.com/api/user.status?handle=sankxt143&from=1&count=500")
                .call()
        )
        .body_mut()
        .read_json()
    );

    let creation_time_threshold = (
        contest.started_on,
        contest.started_on + contest.duration * 60,
    );

    let mut problem_records = HashMap::new();

    for submission in body.result.iter() {
        if creation_time_threshold.0 <= submission.creation_time_seconds
            && submission.creation_time_seconds <= creation_time_threshold.1
        {
            let problem = problems
                .iter()
                .find(|&x| x.uid == submission.problem.get_uid());

            if let Some(problem) = problem {
                problem_records
                    .entry(problem.id)
                    .or_insert((submission.creation_time_seconds, submission.verdict.clone()));
            }
        }

        if submission.creation_time_seconds > creation_time_threshold.1 {
            break;
        }
    }

    for (problem_id, problem_submission_stat) in problem_records.iter() {
        warp_err!(
            update_contest_problem_map(&mut *tx, &contest.id, problem_id, problem_submission_stat)
                .await
        );
    }

    warp_err!(update_contest(&mut *tx, &contest.id, &true).await);
    warp_err!(tx.commit().await);

    println!("Evaluating contest: {}", input.contest_id);
    Ok(warp::reply())
}
