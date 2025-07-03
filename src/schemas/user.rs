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
}

#[graphql_object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id as i32
    }

    fn username(&self) -> &str {
        &self.username
    }

    async fn recent_contest(&self, ctx: &Context) -> FieldResult<Option<Contest>> {
        let mut tx = ctx.db_pool.clone().begin().await?;
        let result = sqlx::query_as!(
            Contest,
            "select id, name from contest where created_for = ? limit 1;",
            self.username
        )
        .fetch_optional(&mut *tx)
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
