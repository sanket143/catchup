use catchup::{
    CatchupRequest,
    context::Context,
    db,
    schemas::root::{MutationRoot, QueryRoot, Schema},
};
use juniper::EmptySubscription;
use juniper::http::GraphQLRequest;
use lambda_http::{Body, Error, Request, Response, http, run, service_fn};
use lazy_static::lazy_static;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    dotenvy::dotenv().ok();

    db::create_db_connection()
        .await
        .expect("Failed to create db connection");
    run(service_fn(lambda_handler)).await?;

    Ok(())
}

async fn lambda_handler(request: Request) -> Result<Response<String>, Error> {
    let catchup_request: CatchupRequest = CatchupRequest::from(&request);
    log::info!("START: {:?}", request.headers().get("cookie"));

    if let Body::Text(text) = request.body() {
        lazy_static! {
            static ref SCHEMA: Schema =
                Schema::new(QueryRoot, MutationRoot, EmptySubscription::new());
        }

        let graphql_request: Result<GraphQLRequest, _> = serde_json::from_str(text);
        let ctx = Context::from_request(&catchup_request).await?;

        request_handler(graphql_request.unwrap(), ctx).await
    } else {
        Ok(Response::builder()
            .status(http::StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body("Nope".into())
            .unwrap())
    }
}

async fn request_handler(request: GraphQLRequest, ctx: Context) -> Result<Response<String>, Error> {
    lazy_static! {
        static ref SCHEMA: Schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription::new());
    }

    let graphql_response = serde_json::to_value(request.execute(&SCHEMA, &ctx).await)?;
    let username = &graphql_response["data"]["createOrLoginUser"]["username"];

    let response = Response::builder()
        .status(http::StatusCode::OK)
        .header("Access-Control-Allow-Origin", "http://localhost:3000")
        .header("Access-Control-Allow-Credentials", "true")
        .header("Content-Type", "application/json")
        .header(
            http::header::SET_COOKIE,
            "username=sankxt143; SameSite=None; HttpOnly; Max-Age=2592000; Secure",
        )
        .body(graphql_response.to_string())?;

    Ok(response)
}
