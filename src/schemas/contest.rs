use juniper::{GraphQLInputObject, graphql_object};
use sqlx::{Row, prelude::FromRow, sqlite::SqliteRow};

use crate::context::Context;

use super::{
    contest_problem_map::ContestProblemMap, problem::Problem, problem_tag_group::ProblemTagGroup,
    user::User,
};

#[derive(Debug)]
pub struct Contest {
    pub id: i64,
    pub name: String,
}

impl Contest {
    pub async fn by_id(ctx: &Context, contest_id: &i64) -> sqlx::Result<Self> {
        let mut tx = ctx.db_pool.begin().await?;

        sqlx::query_as!(
            Self,
            r#"
                select c.id, c.name from contest as c
                where c.id = ?
            "#,
            contest_id
        )
        .fetch_one(&mut *tx)
        .await
    }

    pub async fn create<'e, E>(
        tx: E,
        input: &CreateContestInput,
        duration: &i64,
        user: &User,
        problem_tag: &ProblemTagGroup,
    ) -> sqlx::Result<Self>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        // hard coded for testing
        sqlx::query_as!(
            Self,
            r#"
                insert into contest (name, duration, created_for, fk_problem_tag_group_id, started_on)
                values (?, ?, ?, ?, 1750691608) returning id as "id!", name;
            "#,
            input.name,
            6000000,
            user.username,
            problem_tag.id
        )
        .fetch_one(tx)
        .await
    }

    pub async fn add_problem_by_uid<'e, E>(&self, tx: E, problem_uid: &str) -> sqlx::Result<()>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query!(
            r#"
                insert into contest_problem_map(fk_contest_id, fk_problem_id)
                select ?, p.id
                from problem as p
                where p.uid = ?;
            "#,
            self.id,
            problem_uid,
        )
        .execute(tx)
        .await?;

        Ok(())
    }
}

#[graphql_object(Context = Context)]
impl Contest {
    fn id(&self) -> i32 {
        self.id as i32
    }

    fn name(&self) -> &String {
        &self.name
    }

    async fn problems(&self, ctx: &Context) -> Vec<ContestProblemMap> {
        ContestProblemMap::by_contest_id(ctx, &self.id)
            .await
            .expect("Unable to fetch contest problem map for a Contest")
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Contest creation Input")]
pub struct CreateContestInput {
    pub name: String,
}
