use crate::dataloaders::Dataloaders;
use crate::schemas::user;
use crate::{CatchupRequest, db};
use lambda_http::Request;
use sqlx::PgPool;

pub struct Context {
    pub db_pool: &'static PgPool,
    pub user: Option<user::User>,
    pub datalaoders: Dataloaders,
}

impl juniper::Context for Context {}

impl Context {
    pub async fn from_request(request: &CatchupRequest<'_>) -> Result<Self, lambda_http::Error> {
        let db_pool = db::DBClient::get().pool().await;

        let user = if let Some(context) = &request.context {
            Some(
                user::User::by_username(db_pool, &context.username)
                    .await
                    .unwrap(),
            )
        } else {
            None
        };

        let ctx = Self {
            user,
            db_pool,
            datalaoders: Dataloaders::new(&db_pool),
        };

        Ok(ctx)
    }
}
