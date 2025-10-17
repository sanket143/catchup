#[derive(Debug, sqlx::FromRow)]
pub struct ContestProblemLevel {
    pub id: i64,
    pub level: i64,
    pub duration: i64,
    pub performance: i64,
    pub problem_rating_level_1: i64,
    pub problem_rating_level_2: i64,
    pub problem_rating_level_3: i64,
    pub problem_rating_level_4: i64,
}

impl ContestProblemLevel {
    pub async fn get<'e, E>(tx: E, level: i32) -> sqlx::Result<ContestProblemLevel>
    where
        E: sqlx::PgExecutor<'e>,
    {
        let result = sqlx::query_as::<_, ContestProblemLevel>(
            r#"
            select
                id as "id!", level, duration, performance,
                problem_rating_level_1,
                problem_rating_level_2,
                problem_rating_level_3,
                problem_rating_level_4
            from contest_problem_level as cpl
            where level = $1;
        "#,
        )
        .bind(level)
        .fetch_one(tx)
        .await?;

        Ok(result)
    }
}
