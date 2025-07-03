use sqlx::{Executor, Sqlite};

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
    pub async fn get<'e, E>(tx: E, level: i64) -> sqlx::Result<ContestProblemLevel>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let result = sqlx::query_as_unchecked!(
            ContestProblemLevel,
            r#"
            select
                id, level, duration, performance,
                problem_rating_level_1,
                problem_rating_level_2,
                problem_rating_level_3,
                problem_rating_level_4
            from contest_problem_level as cpl
            where level = ?;
        "#,
            level
        )
        .fetch_one(tx)
        .await?;

        Ok(result)
    }
}
