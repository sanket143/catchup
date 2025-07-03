use actix_web::{Error, HttpResponse, Responder, get, route, web};
use juniper::http::{GraphQLRequest, graphiql::graphiql_source};

use crate::{
    context::Context,
    db::Pool,
    schemas::root::{Schema, create_schema},
};

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

pub fn register(config: &mut web::ServiceConfig) {
    config.service(graphql).service(graphql_playground);
}
