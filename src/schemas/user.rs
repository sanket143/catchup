use juniper::{FieldResult, GraphQLInputObject, graphql_object};
use sqlx::{Executor, prelude::FromRow};

use super::contest::Contest;
use crate::context::Context;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub level: i64,
    pub username: String,
}

impl User {
    pub async fn by_username<'e, E>(tx: E, username: &str) -> sqlx::Result<Self>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as::<_, Self>(
            r#"select id as "id!", username, level from user as u where u.username = ?"#,
        )
        .bind(username)
        .fetch_one(tx)
        .await
    }
}

impl User {
    pub async fn create<'e, E>(tx: E, username: &str) -> sqlx::Result<Self>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as::<_, Self>(
            r#"insert into user (username) values (?) on conflict (username) do update set is_deleted = false returning id as "id!", username, level;"#,
        )
        .bind(username)
        .fetch_one(tx)
        .await
    }

    pub async fn update_level<'e, E>(&self, tx: E, level_offset: &i64) -> sqlx::Result<()>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query(
            "update user set level = max(level + ?, 1) where username = ?",
        )
        .bind(*level_offset)
        .bind(self.username.clone())
        .execute(tx)
        .await?;

        Ok(())
    }
}

#[graphql_object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id as i32
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn level(&self) -> i32 {
        self.level as i32
    }

    async fn recent_contest(&self, ctx: &Context) -> FieldResult<Option<Contest>> {
        let result = sqlx::query_as::<_, Contest>(
            r#"
                select
                    c.id as "id!", c.name, c.duration, c.level, c.created_on,
                    c.started_on, c.created_for, c.fk_problem_tag_group_id,
                    c.is_evaluated
                from contest as c
                where c.created_for = ?
                order by created_on desc
                limit 1;
            "#,
        )
        .bind(self.username.clone())
        .fetch_optional(&*ctx.db_pool)
        .await?;

        Ok(result)
    }

    async fn contests(
        &self,
        ctx: &Context,
        filters: Option<UserContestFilter>,
    ) -> FieldResult<Vec<Contest>> {
        let mut tx = ctx.db_pool.clone().begin().await?;
        let result = sqlx::query_as::<_, Contest>(
            r#"
                select
                    c.id as "id!", c.name, c.duration, c.level, c.created_on,
                    c.started_on, c.created_for, c.fk_problem_tag_group_id,
                    c.is_evaluated
                from contest as c
                where c.created_for = ?
                order by created_on desc;
            "#,
        )
        .bind(self.username.clone())
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(result)
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub username: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User.contests filter")]
pub struct UserContestFilter {
    pub ids: Option<Vec<i32>>,
}
