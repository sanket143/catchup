use serde::Deserialize;
use warp::{reject::Rejection, reply::Reply};

use crate::state::AppState;

#[derive(Deserialize)]
struct Response {
    result: ResponseResult,
}

#[derive(Deserialize, Debug)]
struct Problem {
    #[serde(rename = "contestId")]
    contest_id: usize,
    index: String,
    name: String,
    tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ResponseResult {
    problems: Vec<Problem>,
}

pub async fn handler(state: AppState) -> Result<impl Reply, Rejection> {
    println!("Syncing Codeforces problems...");
    let body: Response = ureq::get("https://codeforces.com/api/problemset.problems")
        .call()
        .map_err(|err| {
            println!("{:?}", err);
            warp::reject()
        })?
        .body_mut()
        .read_json()
        .map_err(|err| {
            println!("{:?}", err);
            warp::reject()
        })?;

    // chunk size so that it never exceeds the bindings limit
    for chunk in body.result.problems.chunks(1000) {
        let mut sql = String::from(
            "INSERT INTO problem (platform_uid, fk_platform_id, title, url, metadata) VALUES ",
        );
        for problem in chunk.iter() {
            sql.push_str(&format!(
                "('{}/{}', 1, '{}', 'https://codeforces.com/problemset/problem/{}/{}', '{}'),",
                problem.contest_id,
                problem.index,
                problem.name.replace("'", "''"),
                problem.contest_id,
                problem.index,
                serde_json::to_value(&problem.tags).unwrap()
            ));
        }

        sql = sql.trim_matches(',').to_owned();

        let result = sqlx::query(&sql)
            .execute(&state.db_pool)
            .await
            .map_err(|err| {
                println!("ERR: {:?}", err);
                warp::reject()
            })?;

        println!("{:?}", result);
    }

    println!("Sync complete!");
    Ok(warp::reply())
}
