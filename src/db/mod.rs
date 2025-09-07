use once_cell::sync::OnceCell;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::postgres::PgPoolOptions;

static POSTGRESQL_DB: OnceCell<Pool<Postgres>> = OnceCell::new();
static DB_CLIENT: DBClient = DBClient {};

#[derive(Clone, Copy)]
pub struct DBClient;

impl DBClient {
    pub fn get() -> &'static DBClient {
        &DB_CLIENT
    }

    pub async fn pool(self) -> &'static sqlx::Pool<Postgres> {
        POSTGRESQL_DB.get().unwrap()
    }
}

pub async fn create_db_connection() -> Result<(), sqlx::Error> {
    POSTGRESQL_DB
        .set({
            let uri = std::env::var("DATABASE_URL").unwrap();
            PgPoolOptions::new()
                .max_connections(4)
                .connect(uri.as_str())
                .await?
        })
        .unwrap();

    Ok(())
}
