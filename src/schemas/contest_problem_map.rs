use juniper::graphql_object;
use serde::Serialize;

use crate::context::Context;

use super::{contest::Contest, problem::Problem};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ContestProblemMap {
    pub id: i64,
    pub fk_contest_id: i64,
    pub fk_problem_id: i64,
    pub latest_submission_at: i64,
    pub is_evaluated: bool,
    pub verdict: String,
}

#[graphql_object(Context = Context)]
impl ContestProblemMap {
    fn id(&self) -> i32 {
        self.id as i32
    }

    async fn problem(&self, ctx: &Context) -> Problem {
        Problem::by_id(ctx, &self.fk_problem_id)
            .await
            .expect("Unable to find problem for this ContestProblemMap")
    }

    async fn contest(&self, ctx: &Context) -> Contest {
        Contest::by_id(ctx, &self.fk_contest_id)
            .await
            .expect("Unable to find problem for this ContestProblemMap")
    }

    fn latest_submission_at(&self) -> i32 {
        self.latest_submission_at as i32
    }

    fn is_evaluated(&self) -> &bool {
        &self.is_evaluated
    }

    fn verdict(&self) -> &String {
        &self.verdict
    }
}

impl ContestProblemMap {
    pub async fn by_contest_id(ctx: &Context, contest_id: &i64) -> sqlx::Result<Vec<Self>> {
        let mut tx = ctx.db_pool.begin().await?;

        sqlx::query_as!(
            Self,
            r#"
                select cpm.id as "id!",
                    cpm.fk_contest_id,
                    cpm.fk_problem_id,
                    cpm.latest_submission_at as "latest_submission_at!",
                    cpm.is_evaluated as "is_evaluated!",
                    cpm.verdict as "verdict!"
                from contest_problem_map as cpm
                where cpm.fk_contest_id = ?
            "#,
            contest_id
        )
        .fetch_all(&mut *tx)
        .await
    }
}
