use juniper::graphql_object;
use serde::Serialize;

use crate::context::Context;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Problem {
    pub id: i32,
    pub uid: String,
    pub title: String,
    pub url: String,
    pub rating: Option<i32>,
}

#[graphql_object(Context = Context)]
impl Problem {
    fn id(&self) -> i32 {
        self.id as i32
    }
    fn uid(&self) -> &String {
        &self.uid
    }
    fn title(&self) -> &String {
        &self.title
    }

    fn url(&self) -> &String {
        &self.url
    }

    fn rating(&self) -> Option<i32> {
        self.rating.map(|x| x as i32)
    }
}

impl Problem {
    pub async fn by_id(ctx: &Context, id: &i32) -> sqlx::Result<Self> {
        let mut tx = ctx.db_pool.begin().await?;

        sqlx::query_as::<_, Self>(
            r#"select id, uid, title, url, rating from problem where id = $1"#,
        )
        .bind(*id)
        .fetch_one(&mut *tx)
        .await
    }
}
