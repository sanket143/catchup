use futures::stream::{self, StreamExt, TryStreamExt};
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    context::Context,
    schemas::{
        codeforces,
        contest::{Contest, CreateContestInput, EndContestInput, EvaluateContestInput},
        contest_problem_level::ContestProblemLevel,
        contest_problem_map::ContestProblemMap,
        problem::Problem,
        problem_tag_group::ProblemTagGroup,
    },
};

pub async fn create(ctx: &Context, input: &CreateContestInput) -> sqlx::Result<Contest> {
    use std::time::{SystemTime, UNIX_EPOCH};

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

    // Use random number to decide how many problems will there be in the contest
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let number_of_problems = match nanos % 100 {
        0..10 => 1,
        10..30 => 2,
        30..60 => 3,
        _ => 4,
    };

    for rating in [
        contest_problem_level.problem_rating_level_1,
        contest_problem_level.problem_rating_level_2,
        contest_problem_level.problem_rating_level_3,
        contest_problem_level.problem_rating_level_4,
    ]
    .iter()
    .skip(4 - number_of_problems)
    {
        contest
            .add_random_problem(&mut *tx, rating, &problem_tag_group)
            .await?;
    }

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

    // if contest is already evaluated, then no need to proceed further
    if contest.is_evaluated {
        return Ok(contest);
    }

    let user = ctx.user.as_ref().unwrap();
    let mut tx = ctx.db_pool.begin().await?;

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

    let mut level_offset = if problem_records.len() >= problems.len() {
        1
    } else {
        -1
    };

    let mut problems_are_still_under_testing = false;

    for (problem_id, problem_submission_stat) in problem_records.iter() {
        if problem_submission_stat.1 != "OK" {
            level_offset = -1;
        }

        if problem_submission_stat.1 == "TESTING" {
            problems_are_still_under_testing = true;
        }

        // TODO: should be doable using contest_problem_map.evaluate(...) like API
        ContestProblemMap::update_evaluation_stats(
            &mut *tx,
            &contest.id,
            problem_id,
            problem_submission_stat,
        )
        .await?;
    }

    // Don't update evaluation status and user's level if submissions are still under testing
    if problems_are_still_under_testing {
        return Ok(contest);
    }

    contest.mark_as_evaluate(&mut *tx).await?;
    user.update_level(&mut *tx, &level_offset).await?;

    tx.commit().await?;

    Ok(contest)
}

pub async fn end(ctx: &Context, input: &EndContestInput) -> sqlx::Result<Contest> {
    // this mutation might never be used as of now, use evaluate mutation to end the contest
    // evaluated contest will also be considered as ended
    evaluate(
        ctx,
        &EvaluateContestInput {
            contest_id: input.contest_id,
        },
    )
    .await
}
