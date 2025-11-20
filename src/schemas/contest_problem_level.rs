#[derive(Debug, sqlx::FromRow)]
pub struct ContestProblemLevel {
    pub id: i32,
    pub level: i32,
    pub duration: i32,
    pub performance: i32,
    pub problem_rating_level_1: i32,
    pub problem_rating_level_2: i32,
    pub problem_rating_level_3: i32,
    pub problem_rating_level_4: i32,
}

impl ContestProblemLevel {
    pub async fn get<'e, E>(tx: E, level: i32) -> sqlx::Result<ContestProblemLevel>
    where
        E: sqlx::PgExecutor<'e>,
    {
        let result = sqlx::query_as::<_, ContestProblemLevel>(
            r#"
            select
                cpl.id, level, duration, performance,
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
