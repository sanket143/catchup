use juniper::{
    EmptySubscription, FieldError, FieldResult, RootNode, graphql_object, graphql_value,
};

use super::{
    contest::{Contest, CreateContestInput},
    user::{User, UserInput},
};
use crate::{
    context::Context,
    controllers::{self, problem_list::sync_problem_list},
    db::Pool,
};

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Get Single user reference by user ID")]
    async fn user(username: String, context: &Context) -> FieldResult<User> {
        let mut tx = context.db_pool.clone().begin().await?;
        let result = sqlx::query_as!(
            User,
            r#"select id as "id!", username, level from user where username = ? limit 1;"#,
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
    #[graphql(
        description = "Create a user with given username, if it already exists then just return the user"
    )]
    async fn create_or_login_user(ctx: &Context, input: UserInput) -> FieldResult<User> {
        let mut tx = ctx.db_pool.clone().begin().await?;
        let user = User::by_username(&mut *tx, &input.username).await?;

        tx.commit().await?;

        Ok(user)
    }

    #[graphql(description = "Create new local contest")]
    async fn create_contest(ctx: &Context, input: CreateContestInput) -> FieldResult<Contest> {
        let contest = controllers::contest::create(ctx, &input).await?;
        Ok(contest)
    }

    #[graphql(description = "Create new local contest")]
    async fn sync_problem_list(ctx: &Context) -> FieldResult<bool> {
        Ok(sync_problem_list(ctx).await?)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
