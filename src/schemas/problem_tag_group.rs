use futures::stream::{self, StreamExt, TryStreamExt};
use juniper::{FieldResult, graphql_object};

use crate::context::Context;

use super::contest::Contest;

pub struct ProblemTagGroup {
    pub id: i64,
    pub name: String,
}

#[graphql_object(Context = Context)]
impl ProblemTagGroup {
    fn id(&self) -> i32 {
        self.id as i32
    }

    fn name(&self) -> &String {
        &self.name
    }

    async fn contests(&self, ctx: &Context) -> FieldResult<Vec<Contest>> {
        let user = ctx.user.as_ref().expect("User not logged in");
        let contest_ids = stream::iter(
            sqlx::query!(
                r#"
                    select c.id from problem_tag_group as ptg
                    join contest as c
                    on c.fk_problem_tag_group_id = ptg.id
                    and c.is_deleted = false
                    where ptg.id = ?
                    and c.created_for = ?
                "#,
                self.id,
                user.username
            )
            .fetch_all(&*ctx.db_pool)
            .await?,
        )
        .map(|x| async move { Contest::by_id(ctx, &x.id).await });

        let contests: Vec<Contest> = contest_ids.buffer_unordered(10).try_collect().await?;

        Ok(contests)
    }
}

impl ProblemTagGroup {
    pub async fn get_random<'e, E>(tx: E) -> Result<Self, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        let result = sqlx::query_as!(
            Self,
            r#"
            select id, name from problem_tag_group
            order by random() limit 1
        "#
        )
        .fetch_one(tx)
        .await?;

        Ok(result)
    }

    pub async fn by_id(ctx: &Context, problem_tag_group_id: &i64) -> Result<Self, sqlx::Error> {
        let mut tx = ctx.db_pool.begin().await?;

        sqlx::query_as!(
            Self,
            r#"
                select id, name from problem_tag_group
                where id = ? limit 1
            "#,
            problem_tag_group_id
        )
        .fetch_one(&mut *tx)
        .await
    }
}
