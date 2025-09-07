use std::path::PathBuf;

use actix_files::{self as af, NamedFile};
use actix_web::{Error, HttpRequest, HttpResponse, Responder, Result, get, route, web};
use juniper::http::{GraphQLRequest, graphiql::graphiql_source};

use crate::{context::Context, schemas::root::Schema};

/// GraphiQL UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    web::Html::new(graphiql_source("/graphql", None))
}

async fn spa_index(_req: HttpRequest) -> Result<NamedFile> {
    let index_path: PathBuf = "./web/dist/index.html".parse().unwrap();
    Ok(NamedFile::open(index_path)?)
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .service(graphql_playground)
        .service(af::Files::new("/", "./web/dist").index_file("index.html"))
        .default_service(web::get().to(spa_index));
}
