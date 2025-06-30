use juniper::{
    EmptySubscription, FieldError, FieldResult, RootNode, graphql_object, graphql_value,
};

use super::user::{User, UserInput};
use crate::db::Pool;

pub struct Context {
    pub db_pool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Get Single user reference by user ID")]
    async fn user(username: String, context: &Context) -> FieldResult<User> {
        let mut tx = context.db_pool.clone().begin().await?;
        let result = sqlx::query_as_unchecked!(
            User,
            "select id, username from user where username = ?;",
            username
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(result)
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_or_login_user(context: &Context, input: UserInput) -> FieldResult<User> {
        let mut tx = context.db_pool.clone().begin().await?;
        let user = sqlx::query_as!(
            User,
            "insert into user (username) values (?) on conflict (username) do update set is_deleted = false returning id, username;",
            input.username
        )
        .fetch_one(&mut *tx)
        .await?;

        Ok(user)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
