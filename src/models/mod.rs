use serde::Deserialize;

#[derive(Debug)]
pub struct User {
    username: String,
}

impl User {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

#[derive(Deserialize)]
pub struct Response {
    pub result: ResponseResult,
}

#[derive(Deserialize, Debug)]
pub struct Problem {
    #[serde(rename = "contestId")]
    pub contest_id: usize,
    pub index: String,
    pub name: String,
    pub tags: Vec<String>,
    pub rating: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseResult {
    pub problems: Vec<Problem>,
}
