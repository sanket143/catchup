use serde_json::json;
use warp::{reject::Rejection, reply::Reply};

use crate::{models::Response, state::AppState};

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
            let metadata = json!({
                "tags": serde_json::to_value(&problem.tags).unwrap(),
                "rating": serde_json::to_value(&problem.rating).unwrap()
            });

            sql.push_str(&format!(
                "('CF/{}/{}', 1, '{}', 'https://codeforces.com/problemset/problem/{}/{}', '{}'),",
                problem.contest_id,
                problem.index,
                problem.name.replace("'", "''"),
                problem.contest_id,
                problem.index,
                metadata
            ));
        }

        sql = sql.trim_matches(',').to_owned();
        sql.push_str("on conflict (platform_uid) do update set title = EXCLUDED.title, url = EXCLUDED.url, metadata = EXCLUDED.metadata");

        sqlx::query(&sql)
            .execute(&state.db_pool)
            .await
            .map_err(|err| {
                println!("ERR: {:?}", err);
                warp::reject()
            })?;
    }

    println!("Sync complete!");
    Ok(warp::reply())
}
