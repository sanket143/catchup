use juniper::{GraphQLInputObject, graphql_object};
use sqlx::{Row, prelude::FromRow, sqlite::SqliteRow};

use super::root::Context;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
}

impl<'r> FromRow<'r, SqliteRow> for User {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            username: row.get("username"),
        })
    }
}

#[graphql_object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id as i32
    }

    fn username(&self) -> &str {
        &self.username
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub username: String,
}
