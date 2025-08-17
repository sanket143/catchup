use juniper::{FieldError, FieldResult, graphql_object, graphql_value};
use serde::Serialize;

use crate::context::Context;

use super::{contest::Contest, problem::Problem};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ContestProblemMap {
    pub id: i64,
    pub fk_contest_id: i64,
    pub fk_problem_id: i64,
    pub latest_submission_at: Option<i64>,
    pub is_evaluated: bool,
    pub verdict: String,
}

#[graphql_object(Context = Context)]
impl ContestProblemMap {
    fn id(&self) -> i32 {
        self.id as i32
    }

    async fn problem(&self, ctx: &Context) -> FieldResult<Problem> {
        // TODO: Ideally, it should handled by dataloder
        Problem::by_id(ctx, &self.fk_problem_id).await.map_err(|_| {
            FieldError::new(
                "Unable to find problem for this ContestProblemMap",
                graphql_value!({}),
            )
        })
    }

    async fn contest(&self, ctx: &Context) -> FieldResult<Contest> {
        // TODO: Ideally, it should handled by dataloder
        Contest::by_id(ctx, &self.fk_contest_id).await.map_err(|_| {
            FieldError::new(
                "Unable to find contest for this ContestProblemMap",
                graphql_value!({}),
            )
        })
    }

    fn latest_submission_at(&self) -> Option<i32> {
        self.latest_submission_at.map(|x| x as i32)
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
        sqlx::query_as!(
            Self,
            r#"
                select cpm.id as "id!",
                    cpm.fk_contest_id,
                    cpm.fk_problem_id,
                    cpm.latest_submission_at,
                    cpm.is_evaluated as "is_evaluated!",
                    cpm.verdict as "verdict!"
                from contest_problem_map as cpm
                where cpm.fk_contest_id = ?
            "#,
            contest_id
        )
        .fetch_all(&*ctx.db_pool)
        .await
    }

    // TODO: Should have a &self signature and should not be dependent on contest_id
    // and problem_id
    pub async fn update_evaluation_stats<'e, E>(
        tx: E,
        contest_id: &i64,
        problem_id: &i64,
        stat: &(i64, String),
    ) -> sqlx::Result<()>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query!(
            r#"
            update contest_problem_map as cpm
            set is_evaluated = true,
            latest_submission_at = ?,
            verdict = ?
            where cpm.fk_contest_id = ? and cpm.fk_problem_id = ?;
        "#,
            stat.0,
            stat.1,
            contest_id,
            problem_id
        )
        .execute(tx)
        .await?;

        Ok(())
    }
}
