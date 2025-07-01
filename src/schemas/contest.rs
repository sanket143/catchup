use juniper::{GraphQLInputObject, graphql_object};
use sqlx::{Row, prelude::FromRow, sqlite::SqliteRow};

use super::root::Context;

#[derive(Debug)]
pub struct Contest {
    pub id: i64,
    pub name: String,
}

#[graphql_object(Context = Context)]
impl Contest {
    fn id(&self) -> i32 {
        self.id as i32
    }
}
