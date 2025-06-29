use sqlx::{Executor, Sqlite};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub level: i64,
}

pub async fn create_user<'e, E>(tx: E, username: &str) -> sqlx::Result<User>
where
    E: Executor<'e, Database = Sqlite>,
{
    let result = sqlx::query_as!(
        User,
        r#"
            insert into user (username) values (?)
            on conflict (username) do update set is_deleted = false
            returning id, username, level;
        "#,
        username
    )
    .fetch_one(tx)
    .await?;

    Ok(result)
}
