use juniper::{FieldResult, GraphQLInputObject, graphql_object};
use sqlx::Executor;

use super::contest::Contest;
use crate::context::Context;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub level: i64,
    pub username: String,
}

impl User {
    pub async fn by_username<'e, E>(tx: E, username: &str) -> sqlx::Result<Self>
    where
        E: Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query_as!(
            Self,
            r#"select id as "id!", username, level from user as u where u.username = ?"#,
            username
        )
        .fetch_one(tx)
        .await
    }
}

impl User {
    pub async fn create<'e, E>(tx: E, username: &str) -> sqlx::Result<Self>
    where
        E: Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query_as!(
            Self,
            "insert into user (username) values (?) on conflict (username) do update set is_deleted = false returning id, username, level;",
            username
        )
        .fetch_one(tx)
        .await
    }

    pub async fn update_level<'e, E>(&self, tx: E, level_offset: &i64) -> sqlx::Result<()>
    where
        E: Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query!(
            "update user set level = max(level + ?, 1) where username = ?",
            level_offset,
            self.username
        )
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
        let result = sqlx::query_as!(
            Contest,
            r#"
                select
                    c.id, c.name, c.duration, c.created_on,
                    c.started_on, c.created_for, c.fk_problem_tag_group_id,
                    c.is_evaluated
                from contest as c
                where c.created_for = ?
                order by created_on desc
                limit 1;
            "#,
            self.username
        )
        .fetch_optional(&*ctx.db_pool)
        .await?;

        Ok(result)
    }

    async fn contests(&self, ctx: &Context) -> FieldResult<Vec<Contest>> {
        let mut tx = ctx.db_pool.clone().begin().await?;
        let result = sqlx::query_as!(
            Contest,
            r#"
                select
                    c.id, c.name, c.duration, c.created_on,
                    c.started_on, c.created_for, c.fk_problem_tag_group_id,
                    c.is_evaluated
                from contest as c
                where c.created_for = ?
                order by created_on desc;
            "#,
            self.username
        )
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
