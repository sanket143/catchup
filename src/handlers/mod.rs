use std::path::PathBuf;

use actix_files::{self as af, NamedFile};
use actix_web::{Error, HttpRequest, HttpResponse, Responder, Result, get, route, web};
use juniper::http::{GraphQLRequest, graphiql::graphiql_source}; // Alias for convenience

use crate::{context::Context, schemas::root::Schema};

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    ctx: Context,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

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
        .service(graphql)
        .service(graphql_playground)
        .service(af::Files::new("/", "./web/dist").index_file("index.html"))
        .default_service(web::get().to(spa_index));
}
