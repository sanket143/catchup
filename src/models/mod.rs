pub mod codeforces;
pub mod contest;
pub mod contest_problem_level;
pub mod contest_problem_map;
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
