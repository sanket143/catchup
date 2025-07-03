use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use sqlx::{Executor, Sqlite, Transaction};

use crate::schemas::contest::Contest;

use super::{problem::Problem, problem_tag_group::ProblemTagGroup};

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

pub async fn get_contest_id<'e, E>(tx: E, contest_id: &i64) -> sqlx::Result<Contest>
where
    E: Executor<'e, Database = Sqlite>,
{
    let result = sqlx::query_as_unchecked!(
        Contest,
        r#"
            select id, name from contest as c
            where c.id = ?
        "#,
        contest_id
    )
    .fetch_one(tx)
    .await?;

    Ok(result)
}

pub async fn create_contest<'e, E>(
    tx: E,
    name: &str,
    duration: &i64,
    username: &str,
    problem_tag: &ProblemTagGroup,
) -> sqlx::Result<Contest>
where
    E: Executor<'e, Database = Sqlite>,
{
    // hard coded for testing
    let result = sqlx::query_as_unchecked!(
        Contest,
        r#"
            insert into contest (name, duration, created_for, fk_problem_tag_group_id, started_on)
            values (?, ?, ?, ?, 1750691608) returning id, name;
        "#,
        name,
        6000000,
        username,
        problem_tag.id
    )
    .fetch_one(tx)
    .await?;

    Ok(result)
}

pub async fn update_contest<'e, E>(tx: E, contest_id: &i64, is_evaluated: &bool) -> sqlx::Result<()>
where
    E: Executor<'e, Database = Sqlite>,
{
    sqlx::query!(
        r#"
            update contest set is_evaluated = ?
            where id = ?;
        "#,
        is_evaluated,
        contest_id
    )
    .execute(tx)
    .await?;

    Ok(())
}

pub async fn add_problem_in_contest<'e, E>(
    tx: E,
    contest_id: &i64,
    problem_rating: &i64,
    problem_tag_group: &ProblemTagGroup,
) -> sqlx::Result<()>
where
    E: Executor<'e, Database = Sqlite>,
{
    sqlx::query!(
        r#"
            insert into contest_problem_map(fk_contest_id, fk_problem_id)
            select ?, p.id
            from problem_tag_group as ptg
            join problem_tag as pt
            on pt.fk_problem_tag_group_id = ptg.id
            join problem_tag_map as ptm
            on ptm.fk_problem_tag_id = pt.id
            join problem as p
            on p.id = ptm.fk_problem_id
            and p.rating = ?
            left join contest_problem_map as cpm
            on cpm.fk_problem_id = p.id
            and cpm.fk_contest_id = ?
            and cpm.is_deleted = false
            where ptg.id = ?
            and cpm.id is null
            order by random() limit 1;
        "#,
        contest_id,
        problem_rating,
        contest_id,
        problem_tag_group.id,
    )
    .execute(tx)
    .await?;

    Ok(())
}

// only for testing
pub async fn add_problem_in_contest_by_id<'e, E>(
    tx: E,
    contest_id: &i64,
    uid: &str,
) -> sqlx::Result<()>
where
    E: Executor<'e, Database = Sqlite>,
{
    sqlx::query!(
        r#"
            insert into contest_problem_map(fk_contest_id, fk_problem_id)
            select ?, p.id
            from problem as p
            where p.uid = ?;
        "#,
        contest_id,
        uid,
    )
    .execute(tx)
    .await?;

    Ok(())
}

// pub async fn current_contest<'e, E>(
//     tx: E,
//     username: &str,
// ) -> sqlx::Result<Option<Contest>, sqlx::Error>
// where
//     E: Executor<'e, Database = Sqlite>,
// {
//     let now = SystemTime::now();

//     let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();

//     // Get epoch time in seconds
//     let epoch = duration_since_epoch.as_secs() as i64;
//     let result = sqlx::query_as!(
//         Contest,
//         r#"
//             select id, name, duration, created_on, started_on, fk_problem_tag_group_id from contest
//             where (contest.started_on + (duration * 60)) >= ? and created_for = ? limit 1;
//         "#,
//         epoch,
//         username
//     )
//     .fetch_optional(tx)
//     .await?;

//     Ok(result)
// }
