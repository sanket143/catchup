use sqlx::{Executor, Sqlite, prelude::FromRow};

#[derive(Debug, Clone, FromRow)]
pub struct ProblemTagGroup {
    pub id: i64,
    pub name: String,
}

pub async fn get_random<'e, E>(tx: E) -> Result<ProblemTagGroup, sqlx::Error>
where
    E: Executor<'e, Database = Sqlite>,
{
    let result = sqlx::query_as!(
        ProblemTagGroup,
        r#"
            select id, name from problem_tag_group
            order by random() limit 1
        "#
    )
    .fetch_one(tx)
    .await?;

    Ok(result)
}
