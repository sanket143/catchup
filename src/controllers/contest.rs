use futures::stream::{self, StreamExt, TryStreamExt};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    context::Context,
    schemas::{
        codeforces,
        contest::{Contest, CreateContestInput, EvaluateContestInput},
        contest_problem_level::ContestProblemLevel,
        contest_problem_map::ContestProblemMap,
        problem::Problem,
        problem_tag_group::ProblemTagGroup,
    },
};

pub async fn create(ctx: &Context, input: &CreateContestInput) -> sqlx::Result<Contest> {
    let user = ctx.user.as_ref().unwrap();
    let mut tx = ctx.db_pool.begin().await?;

    let problem_tag_group = ProblemTagGroup::get_random(&mut *tx).await?;
    let contest_problem_level = ContestProblemLevel::get(&mut *tx, user.level).await?;
    let contest = Contest::create(
        &mut *tx,
        &input,
        &contest_problem_level.duration,
        &user,
        &problem_tag_group,
    )
    .await?;

    for rating in [
        contest_problem_level.problem_rating_level_1,
        contest_problem_level.problem_rating_level_2,
        contest_problem_level.problem_rating_level_3,
        contest_problem_level.problem_rating_level_4,
    ]
    .iter()
    {
        contest
            .add_random_problem(&mut *tx, rating, &problem_tag_group)
            .await?;
    }

    // for uid in ["CF/2117/B", "CF/2117/A", "CF/2112/C", "CF/2112/B"].iter() {
    //     contest.add_problem_by_uid(&mut *tx, uid).await?;
    // }

    tx.commit().await?;

    Ok(contest)
}

pub async fn evaluate(ctx: &Context, input: &EvaluateContestInput) -> sqlx::Result<Contest> {
    #[derive(Deserialize)]
    pub struct Response {
        pub result: Vec<codeforces::CodeforcesSubmission>,
    }

    let contest_id = input.contest_id as i64;
    let contest = Contest::by_id(ctx, &contest_id).await?;

    // TODO: Should be handled by single sql query to get problems by problem ids
    let contest_problem_map = ContestProblemMap::by_contest_id(ctx, &contest_id).await?;
    let problems = stream::iter(contest_problem_map)
        .map(|x| async move { Problem::by_id(ctx, &x.fk_problem_id).await });
    let problems: Vec<Problem> = problems.buffer_unordered(10).try_collect().await?;

    let body: Response =
        ureq::get("https://codeforces.com/api/user.status?handle=sankxt143&from=1&count=500")
            .call()
            .expect("Failed to fetch submission details from codeforces")
            .body_mut()
            .read_json()
            .expect("Failed to parse the body using JSON parser");

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
        // TODO: should be doable using contest_problem_map.evaluate(...) like API
        ContestProblemMap::update_evaluation_stats(
            ctx,
            &contest.id,
            problem_id,
            problem_submission_stat,
        )
        .await?;
    }

    Ok(contest)
}
