use serde::Deserialize;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{reject::Rejection, reply::Reply};

use crate::state::AppState;

#[derive(Deserialize, Debug)]
pub struct ListProblemInput {
    #[serde(rename = "contestId")]
    contest_id: Option<usize>,
}

pub async fn list_problems(
    state: Arc<RwLock<AppState>>,
    input: ListProblemInput,
) -> Result<impl Reply, Rejection> {
    println!("{:?}", input);

    let result = sqlx::query(
        "
            select p.*
            from problem as p
            where json_extract(p.metadata, '$.rating') = 800 and p.fk_platform_id = 1
            order by random()
            limit 4;
        ",
    )
    .fetch_all(&state.read().await.db_pool)
    .await
    .map_err(|err| {
        println!("ERR: {:?}", err);
        warp::reject()
    })?;

    let problems: Vec<serde_json::Value> = result
        .iter()
        .map(|row| {
            let url: String = row.get("url");
            let platform_uid: String = row.get("platform_uid");
            let title: String = row.get("title");

            serde_json::json!({
                "url": url,
                "platform_uid": platform_uid,
                "title": title
            })
        })
        .collect();

    println!("Sync complete!");
    Ok(warp::reply())
}
