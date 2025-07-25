use std::collections::{HashMap, HashSet};

use serde::Deserialize;
use sqlx::{QueryBuilder, Row};

use crate::context::Context;

const BIND_LIMIT: usize = 32766;

#[derive(Deserialize, Debug)]
struct CodeforcesProblem {
    #[serde(rename = "contestId")]
    pub contest_id: u32,
    pub index: String,
    pub name: String,
    pub tags: Vec<String>,
    pub rating: Option<u32>,
}

impl CodeforcesProblem {
    pub fn get_uid(&self) -> String {
        format!("CF/{}/{}", self.contest_id, self.index)
    }
}

#[derive(Deserialize)]
struct Response {
    pub result: ResponseResult,
}

#[derive(Deserialize, Debug)]
struct ResponseResult {
    pub problems: Vec<CodeforcesProblem>,
}

pub async fn sync_problem_list(ctx: &Context) -> sqlx::Result<bool> {
    let mut tx = ctx.db_pool.begin().await?;

    println!("Syncing Codeforces problems...");
    let body: Response = ureq::get("https://codeforces.com/api/problemset.problems")
        .call()
        .expect("Failed to fetch problems from codeforces")
        .body_mut()
        .read_json()
        .expect("Failed to fetch problems from codeforces");

    let mut query_builder: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new("");
    let mut tags = HashSet::new();
    let mut problem_tags_map = vec![];

    // chunk size so that it never exceeds the bindings limit
    for chunk in body.result.problems.chunks(BIND_LIMIT / 6) {
        query_builder.reset();
        query_builder
            .push("insert into problem (uid, fk_platform_id, title, url, rating, metadata) ");

        query_builder.push_values(chunk, |mut b, problem| {
            problem.tags.iter().for_each(|tag| {
                tags.insert(tag.clone());
            });

            let metadata = serde_json::json!({
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

    let mut problem_tag_query_builder: QueryBuilder<sqlx::Sqlite> =
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

    let mut problem_tag_map_query_builder: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new("");

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
        problem_tag_map_query_builder
            .build()
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    println!("Sync complete!");

    Ok(true)
}
