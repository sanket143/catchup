use juniper::{FieldResult, GraphQLInputObject, graphql_object};

use super::{contest::Contest, root::Context};

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[graphql_object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id as i32
    }

    fn username(&self) -> &str {
        &self.username
    }

    async fn recent_contest(&self, ctx: &Context) -> FieldResult<Option<Contest>> {
        let mut tx = ctx.db_pool.clone().begin().await?;
        let result = sqlx::query_as!(
            Contest,
            "select id, name from contest where created_for = ? limit 1;",
            self.username
        )
        .fetch_optional(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(result)
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub username: String,
}
