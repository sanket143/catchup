use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Submission {
    pub problem: Problem,
    pub verdict: String,
    #[serde(rename = "creationTimeSeconds")]
    pub creation_time_seconds: i64,
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
