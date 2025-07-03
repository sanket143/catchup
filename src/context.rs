use crate::schemas::user::{self, User};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, web};
use futures::future::BoxFuture;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct Context {
    pub db_pool: Arc<SqlitePool>,
    pub user: Option<user::User>,
}

impl juniper::Context for Context {}

impl FromRequest for Context {
    type Error = actix_web::Error;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let pool_data = req.app_data::<web::Data<Arc<SqlitePool>>>()
            .expect("DB Pool not configured in app_data. Make sure to call .app_data(web::Data::new(pool.clone()))");
        let pool = pool_data.get_ref().clone(); // Clone the Arc<SqlitePool>
        let username = req.cookie("username").map(|c| c.value().to_string());

        Box::pin(async move {
            let mut user = None;

            if let Some(username) = username.map(|x| (!x.is_empty()).then_some(x)).flatten() {
                user = Some(User::by_username(&*pool, &username).await.expect(&format!(
                    "User with {} username doesn't exist, please login again with this username.",
                    username
                )));
            }

            Ok(Context {
                user,
                db_pool: pool,
            })
        })
    }
}
