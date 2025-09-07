use catchup::{
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
    dotenvy::dotenv().ok();

    db::create_db_connection()
        .await
        .expect("Failed to create db connection");
    run(service_fn(lambda_handler)).await?;

    Ok(())
}

async fn lambda_handler(request: Request) -> Result<Response<String>, Error> {
    println!("START: {:?}", request.uri());

    if let Body::Text(text) = request.body() {
        lazy_static! {
            static ref SCHEMA: Schema =
                Schema::new(QueryRoot, MutationRoot, EmptySubscription::new());
        }

        let graphql_request: Result<GraphQLRequest, _> = serde_json::from_str(text);
        let ctx = Context::from_request(&request).await;

        request_handler(graphql_request.unwrap(), ctx).await
    } else {
        Ok(Response::builder()
            .status(http::StatusCode::BAD_REQUEST)
            .header("Content-Type", "text/plain")
            .body("Internal server error".into())
            .unwrap())
    }
}

async fn request_handler(request: GraphQLRequest, ctx: Context) -> Result<Response<String>, Error> {
    lazy_static! {
        static ref SCHEMA: Schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription::new());
    }

    let graphql_response = serde_json::to_value(request.execute(&SCHEMA, &ctx).await);

    let response = match graphql_response {
        Ok(response) => {
            Response::builder()
                .status(http::StatusCode::OK)
                .header("Content-Type", "application/json")
                // .header(http::header::SET_COOKIE, response.clone().get_cookies())
                .body(response.to_string())
                .unwrap()
        }
        Err(err) => {
            println!("{:?}", err);

            Response::builder()
                .status(http::StatusCode::BAD_REQUEST)
                .header("Content-Type", "text/plain")
                .body("Internal server error".into())
                .map_err(Box::new)
                .unwrap()
        }
    };

    Ok(response)
}
