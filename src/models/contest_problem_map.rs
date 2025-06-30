use sqlx::{Executor, Sqlite};

pub async fn update_contest_problem_map<'e, E>(
    tx: E,
    contest_id: &i64,
    problem_id: &i64,
    stat: &(i64, String),
) -> sqlx::Result<()>
where
    E: Executor<'e, Database = Sqlite>,
{
    sqlx::query!(
        r#"
            update contest_problem_map as cpm
            set is_evaluated = true,
            latest_submission_at = ?,
            verdict = ?
            where cpm.fk_contest_id = ? and cpm.fk_problem_id = ?;
        "#,
        stat.0,
        stat.1,
        contest_id,
        problem_id
    )
    .execute(tx)
    .await?;

    Ok(())
}
