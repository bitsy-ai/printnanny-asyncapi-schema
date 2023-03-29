// CurrentJob represents a CurrentJob model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct CurrentJob {
    #[serde(rename = "job")]
    pub job: Box<crate::Job>,
    #[serde(rename = "progress")]
    pub progress: Box<crate::JobProgress>,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl CurrentJob {
    pub fn new(
        job: crate::Job,
        progress: crate::JobProgress,
        state: String,
        error: Option<String>,
    ) -> CurrentJob {
        CurrentJob {
            job: Box::new(job),
            progress: Box::new(progress),
            state,
            error,
        }
    }
}
