// JobStatusChanged represents a JobStatusChanged model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct JobStatusChanged {
    #[serde(rename = "job", skip_serializing_if = "Option::is_none")]
    pub job: Option<Box<crate::Job>>,
    #[serde(rename = "status")]
    pub status: Box<crate::JobStatus>,
}

impl JobStatusChanged {
    pub fn new(job: Option<crate::Job>, status: crate::JobStatus) -> JobStatusChanged {
        JobStatusChanged {
            job: job.map(Box::new),
            status: Box::new(status),
        }
    }
}
