use tera::Context;
use warp::{reject::Rejection, reply::Reply};

use crate::{state::AppState, templates::init::get_tera};

pub async fn handler(state: AppState) -> Result<impl Reply, Rejection> {
    let mut context = Context::new();
    context.insert("title", "Practice");
    context.insert("message", "Hello from Warp and Tera!");

    let rendered = get_tera().render("index.html", &context).map_err(|e| {
        eprintln!("Tera rendering error: {:?}", e);
        warp::reject::reject()
    })?;

    Ok(warp::reply::html(rendered))
}
