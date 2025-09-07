use futures::stream::{self, StreamExt, TryStreamExt};
use juniper::{FieldError, FieldResult, graphql_object, graphql_value};

use crate::context::Context;
use crate::schemas::contest::Contest;
use sqlx::Row;

#[derive(Debug, sqlx::FromRow)]
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
        let user = ctx
            .user
            .as_ref()
            .ok_or(FieldError::new("User is not logged in", graphql_value!({})))?;

        let rows = sqlx::query(
            r#"
                select c.id from problem_tag_group as ptg
                join contest as c
                on c.fk_problem_tag_group_id = ptg.id
                and c.is_deleted = false
                where ptg.id = ?
                and c.created_for = ?
            "#,
        )
        .bind(self.id)
        .bind(user.username.clone())
        .fetch_all(&*ctx.db_pool)
        .await?;

        let contest_ids = stream::iter(
            rows.into_iter().map(|row| {
                let id: i64 = row.try_get("id").unwrap_or_default();
                async move { Contest::by_id(ctx, &id).await }
            })
        );

        let contests: Vec<Contest> = contest_ids.buffer_unordered(10).try_collect().await?;

        Ok(contests)
    }
}

impl ProblemTagGroup {
    pub async fn get_random<'e, E>(tx: E) -> Result<Self, sqlx::Error>
    where
        E: sqlx::PgExecutor<'e>,
    {
        let row = sqlx::query(
            r#"
            select id, name from problem_tag_group
            order by random() limit 1
        "#
        )
        .fetch_one(tx)
        .await?;

        Ok(ProblemTagGroup {
            id: row.try_get("id").unwrap_or_default(),
            name: row.try_get("name").unwrap_or_default(),
        })
    }

    pub async fn by_id(ctx: &Context, problem_tag_group_id: &i64) -> Result<Self, sqlx::Error> {
        let mut tx = ctx.db_pool.begin().await?;

        let row = sqlx::query(
            r#"
                select id, name from problem_tag_group
                where id = ? limit 1
            "#,
        )
        .bind(*problem_tag_group_id)
        .fetch_one(&mut *tx)
        .await?;

        Ok(ProblemTagGroup {
            id: row.try_get("id").unwrap_or_default(),
            name: row.try_get("name").unwrap_or_default(),
        })
    }
}
