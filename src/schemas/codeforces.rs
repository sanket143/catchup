use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CodeforcesSubmission {
    pub problem: CodeforcesProblem,
    pub verdict: String,
    #[serde(rename = "creationTimeSeconds")]
    pub creation_time_seconds: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CodeforcesProblem {
    #[serde(rename = "contestId")]
    pub contest_id: u32,
    pub index: String,
    pub name: String,
    pub tags: Vec<String>,
    pub rating: Option<u32>,
}

impl CodeforcesProblem {
    pub fn get_uid(&self) -> String {
        format!("CF/{}/{}", self.contest_id, self.index)
    }
}
