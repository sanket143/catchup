use crate::db;
use crate::schemas::user;
use lambda_http::Request;
use sqlx::PgPool;

pub struct Context {
    pub db_pool: &'static PgPool,
    pub user: Option<user::User>,
}

impl juniper::Context for Context {}

impl Context {
    pub async fn from_request(_req: &Request) -> Self {
        let db_pool = db::DBClient::get().pool().await;

        Self {
            user: None,
            db_pool,
        }
    }
}
