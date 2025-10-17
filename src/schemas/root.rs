use juniper::{
    EmptySubscription, FieldError, FieldResult, RootNode, graphql_object, graphql_value,
};
use sqlx::Row;

use super::{
    contest::{Contest, CreateContestInput, EndContestInput, EvaluateContestInput},
    problem_tag_group::ProblemTagGroup,
    user::{User, UserInput},
};
use crate::{
    context::Context,
    controllers::{self, problem_list::sync_problem_list},
};

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Get currently logged in user")]
    async fn user(ctx: &Context) -> FieldResult<&User> {
        let user = ctx
            .user
            .as_ref()
            .ok_or(FieldError::new("User is not logged in", graphql_value!({})))?;

        Ok(user)
    }

    #[graphql(
        description = "List of all the problem tag groups, basically a group of Codeforces topics. Refer to migration file in the codebase for list of tag groups."
    )]
    async fn problem_tag_groups(ctx: &Context) -> FieldResult<Vec<ProblemTagGroup>> {
        let result = sqlx::query_as::<_, ProblemTagGroup>(
            r#"select id, name from public.problem_tag_group;"#,
        )
        .fetch_all(&*ctx.db_pool)
        .await?;

        log::info!("{:?}", result);
        log::info!(
            "Time taken to compute the sum: {:?}",
            std::time::Instant::now()
        );

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
        let mut tx = ctx.db_pool.begin().await?;
        let user = User::create(&mut *tx, &input.username)
            .await
            .expect("Failed to create a user");

        tx.commit().await?;

        Ok(user)
    }

    #[graphql(description = "Create new local contest")]
    async fn create_contest(ctx: &Context, input: CreateContestInput) -> FieldResult<Contest> {
        let contest = controllers::contest::create(ctx, &input).await?;
        Ok(contest)
    }

    #[graphql(description = "Evaluate submissions done by the user for given contestId")]
    async fn evaluate_contest(ctx: &Context, input: EvaluateContestInput) -> FieldResult<Contest> {
        let contest = controllers::contest::evaluate(ctx, &input).await?;
        Ok(contest)
    }

    #[graphql(description = "End contest by id")]
    async fn end_contest(ctx: &Context, input: EndContestInput) -> FieldResult<Contest> {
        let contest = controllers::contest::end(ctx, &input).await?;
        Ok(contest)
    }

    #[graphql(description = "Sync problem list from Codeforces")]
    async fn sync_problem_list(ctx: &Context) -> FieldResult<bool> {
        Ok(sync_problem_list(ctx).await?)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
