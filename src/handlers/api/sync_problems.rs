use serde::Deserialize;
use serde_json::json;
use sqlx::Row;
use sqlx::{QueryBuilder, Sqlite};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{reject::Rejection, reply::Reply};
const BIND_LIMIT: usize = 32766;

use crate::models::codeforces::Problem;
use crate::state::AppState;
use crate::warp_err;

#[derive(Deserialize)]
pub struct Response {
    pub result: ResponseResult,
}

#[derive(Deserialize, Debug)]
pub struct ResponseResult {
    pub problems: Vec<Problem>,
}

pub async fn handler(state: Arc<RwLock<AppState>>) -> Result<impl Reply, Rejection> {
    println!("Syncing Codeforces problems...");
    let body: Response = warp_err!(
        warp_err!(ureq::get("https://codeforces.com/api/problemset.problems").call())
            .body_mut()
            .read_json()
    );

    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("");
    let mut tags = HashSet::new();
    let mut problem_tags_map = vec![];

    let mut tx = warp_err!(state.read().await.db_pool.begin().await);

    // chunk size so that it never exceeds the bindings limit
    for chunk in body.result.problems.chunks(BIND_LIMIT / 6) {
        query_builder.reset();
        query_builder
            .push("insert into problem (uid, fk_platform_id, title, url, rating, metadata) ");

        query_builder.push_values(chunk, |mut b, problem| {
            problem.tags.iter().for_each(|tag| {
                tags.insert(tag.clone());
            });

            let metadata = json!({
                "tags": serde_json::to_value(&problem.tags).unwrap(),
                "rating": serde_json::to_value(&problem.rating).unwrap()
            });

            b.push_bind(problem.get_uid())
                .push_bind(1)
                .push_bind(problem.name.clone())
                .push_bind(format!(
                    "https://codeforces.com/problemset/problem/{}/{}",
                    problem.contest_id, problem.index
                ))
                .push_bind(problem.rating)
                .push_bind(metadata);
        });

        query_builder.push(" on conflict (uid) do update set title = EXCLUDED.title, url = EXCLUDED.url, metadata = EXCLUDED.metadata returning id, metadata;");

        let result = query_builder.build().fetch_all(&mut *tx).await.unwrap();

        result.iter().for_each(|x| {
            let id: u32 = x.get("id");
            let metadata: serde_json::Value = x.get("metadata");
            let tags = metadata["tags"].as_array().unwrap().to_vec();

            tags.iter().for_each(|x| {
                problem_tags_map.push((id, x.as_str().unwrap().to_owned()));
            });
        });
    }

    let mut problem_tag_query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("insert into problem_tag (uid) ");

    problem_tag_query_builder.push_values(tags, |mut b, x| {
        b.push_bind(x);
    });

    problem_tag_query_builder
        .push(" on conflict (uid) do update set is_deleted = false returning *;");

    let result = problem_tag_query_builder
        .build()
        .fetch_all(&mut *tx)
        .await
        .unwrap();

    let mut tag_uid_id_map = HashMap::new();
    result.iter().for_each(|x| {
        let id: u32 = x.get("id");
        let uid: String = x.get("uid");

        tag_uid_id_map.insert(uid, id);
    });

    let mut problem_tag_map_query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("");

    for chunk in problem_tags_map.chunks(BIND_LIMIT / 6) {
        problem_tag_map_query_builder.reset();
        problem_tag_map_query_builder
            .push("insert into problem_tag_map (fk_problem_id, fk_problem_tag_id) ");

        problem_tag_map_query_builder.push_values(chunk, |mut b, x| {
            b.push_bind(x.0);
            b.push_bind(tag_uid_id_map.get(&x.1).unwrap());
        });

        problem_tag_map_query_builder
            .push(" on conflict (fk_problem_id, fk_problem_tag_id) do nothing;");
        warp_err!(
            problem_tag_map_query_builder
                .build()
                .execute(&mut *tx)
                .await
        );
    }

    warp_err!(tx.commit().await);

    println!("Sync complete!");
    Ok(warp::reply())
}
