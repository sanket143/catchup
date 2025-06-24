use tera::Context;
use warp::{reject::Rejection, reply::Reply};

use crate::{state::SharedState, templates::init::get_tera};

pub async fn handler(state: SharedState) -> Result<impl Reply, Rejection> {
    println!("sync problems");
    Ok(warp::reply())
}
