use crate::{state::AppState, templates::init::get_tera};
use std::sync::Arc;
use tera::Context;
use tokio::sync::RwLock;
use warp::{reject::Rejection, reply::Reply};

pub async fn handler(state: Arc<RwLock<AppState>>) -> Result<impl Reply, Rejection> {
    let mut context = Context::new();
    context.insert("title", "Settings");
    context.insert("current_page", "settings");

    let rendered = get_tera().render("settings.html", &context).map_err(|e| {
        eprintln!("Tera rendering error: {:?}", e);
        warp::reject::reject()
    })?;

    Ok(warp::reply::html(rendered))
}
