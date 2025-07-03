pub struct ProblemTagGroup {
    pub id: i64,
    pub name: String,
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
}
