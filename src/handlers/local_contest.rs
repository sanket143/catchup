use sqlx::Row;
use std::sync::Arc;
use tera::Context;
use tokio::sync::RwLock;
use warp::{reject::Rejection, reply::Reply};

use crate::{state::AppState, templates::init::get_tera};

pub async fn handler(state: Arc<RwLock<AppState>>) -> Result<impl Reply, Rejection> {
    let mut context = Context::new();
    context.insert("title", "Local Contest");
    context.insert("current_page", "local-contest");

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

    context.insert("problems", &problems);

    let rendered = get_tera()
        .render("local-contest.html", &context)
        .map_err(|e| {
            eprintln!("Tera rendering error: {:?}", e);
            warp::reject::reject()
        })?;

    Ok(warp::reply::html(rendered))
}
