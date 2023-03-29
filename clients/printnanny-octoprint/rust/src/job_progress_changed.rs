// JobProgressChanged represents a JobProgressChanged model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct JobProgressChanged {
    #[serde(rename = "job", skip_serializing_if = "Option::is_none")]
    pub job: Option<Box<crate::Job>>,
    #[serde(rename = "storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<Box<crate::JobProgress>>,
}

impl JobProgressChanged {
    pub fn new(
        job: Option<crate::Job>,
        storage: Option<String>,
        path: Option<String>,
        progress: Option<crate::JobProgress>,
    ) -> JobProgressChanged {
        JobProgressChanged {
            job: job.map(Box::new),
            storage,
            path,
            progress: progress.map(Box::new),
        }
    }
}
