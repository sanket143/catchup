use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Problem {
    pub id: i64,
    pub uid: String,
    pub title: String,
    pub url: String,
    pub rating: Option<i64>,
}
