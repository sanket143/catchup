use serde::Deserialize;

pub mod contest;
pub mod contest_problem_level;
pub mod problem;
pub mod problem_tag_group;
pub mod user;

#[derive(Debug, Clone)]
pub struct User {
    username: String,
}

impl User {
    pub fn new(username: String) -> Self {
        Self { username }
    }

    pub fn get_username(self) -> String {
        self.username
    }
}

#[derive(Deserialize)]
pub struct Response {
    pub result: ResponseResult,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Problem {
    #[serde(rename = "contestId")]
    pub contest_id: u32,
    pub index: String,
    pub name: String,
    pub tags: Vec<String>,
    pub rating: Option<u32>,
}

impl Problem {
    pub fn get_uid(&self) -> String {
        format!("CF/{}/{}", self.contest_id, self.index)
    }
}

#[derive(Deserialize, Debug)]
pub struct ResponseResult {
    pub problems: Vec<Problem>,
}
