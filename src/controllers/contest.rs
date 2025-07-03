use sqlx::{Executor, Sqlite};

use crate::{
    context::Context,
    models::{contest, contest_problem_level},
    schemas::{
        contest::{Contest, CreateContestInput},
        contest_problem_level::ContestProblemLevel,
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
        // warp_err!(add_problem_in_contest(&mut *tx, &contest.id, rating, &problem_tag_group).await);
    }

    for uid in ["CF/2117/B", "CF/2117/A", "CF/2112/C", "CF/2112/B"].iter() {
        contest.add_problem_by_uid(&mut *tx, uid).await?;
    }

    tx.commit().await?;

    Ok(contest)
}
