use juniper::graphql_object;

use crate::context::Context;

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
