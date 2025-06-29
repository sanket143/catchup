use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use sqlx::{Executor, Sqlite, Transaction};

use super::problem::Problem;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Contest {
    pub id: i64,
    pub name: String,
    pub duration: i64,
    pub created_on: i64,
    pub started_on: i64,
}

impl Contest {
    pub async fn get_problems<'e, E>(&self, tx: E) -> Result<Vec<Problem>, sqlx::Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let result = sqlx::query_as!(
            Problem,
            r#"
                select p.id, p.uid, p.title, p.url, p.rating
                from contest as c
                join contest_problem_map as cpm
                on cpm.fk_contest_id = c.id
                and cpm.is_deleted = false
                join problem as p
                on cpm.fk_problem_id = p.id
                where c.id = ?
            "#,
            self.id
        )
        .fetch_all(tx)
        .await?;

        Ok(result)
    }
}

pub async fn create_contest<'e, E>(
    tx: E,
    name: &str,
    duration: &i64,
    username: &str,
) -> sqlx::Result<Contest>
where
    E: Executor<'e, Database = Sqlite>,
{
    let result = sqlx::query_as!(
        Contest,
        r#"
            insert into contest (name, duration, created_for)
            values (?, ?, ?) returning id, name, duration, created_on, started_on;
        "#,
        name,
        duration,
        username
    )
    .fetch_one(tx)
    .await?;

    Ok(result)
}

pub async fn contest_list(
    mut tx: Transaction<'_, Sqlite>,
    created_for: &str,
) -> sqlx::Result<Vec<Contest>> {
    let result = sqlx::query_as!(
        Contest,
        "select id, name, duration, created_on, started_on from contest where contest.created_for = ?", created_for
    )
    .fetch_all(&mut *tx)
    .await?;

    Ok(result)
}

pub async fn contest_count<'e, E>(tx: E, created_for: &str) -> sqlx::Result<usize>
where
    E: Executor<'e, Database = Sqlite>,
{
    let result = sqlx::query_as!(
        Contest,
        "select id, name, duration, created_on, started_on from contest where contest.created_for = ?", created_for
    )
    .fetch_all(tx)
    .await?;

    Ok(result.len())
}

pub async fn add_problem_in_contest<'e, E>(
    tx: E,
    contest_id: &i64,
    problem_rating: &i64,
    problem_tag: &str,
) -> sqlx::Result<()>
where
    E: Executor<'e, Database = Sqlite>,
{
    sqlx::query!(
        r#"
            insert into contest_problem_map(fk_contest_id, fk_problem_id)
            select ?, p.id
            from problem_tag as pt
            join problem_tag_map as ptm
            on ptm.fk_problem_tag_id = pt.id
            join problem as p
            on p.id = ptm.fk_problem_id
            and p.rating = ?
            left join contest_problem_map as cpm
            on cpm.fk_problem_id = p.id
            and cpm.fk_contest_id = ?
            and cpm.is_deleted = false
            where pt.uid = ?
            and cpm.id is null
            order by random() limit 1;
        "#,
        contest_id,
        problem_rating,
        contest_id,
        problem_tag,
    )
    .execute(tx)
    .await?;

    Ok(())
}

pub async fn current_contest<'e, E>(
    tx: E,
    username: &str,
) -> sqlx::Result<Option<Contest>, sqlx::Error>
where
    E: Executor<'e, Database = Sqlite>,
{
    let now = SystemTime::now();

    let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();

    // Get epoch time in seconds
    let epoch = duration_since_epoch.as_secs() as i64;
    let result = sqlx::query_as!(
        Contest,
        r#"
            select id, name, duration, created_on, started_on from contest
            where (contest.started_on + (duration * 60)) >= ? and created_for = ? limit 1; 
        "#,
        epoch,
        username
    )
    .fetch_optional(tx)
    .await?;

    Ok(result)
}
