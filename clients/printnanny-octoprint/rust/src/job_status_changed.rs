// JobStatusChanged represents a JobStatusChanged model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct JobStatusChanged {
    #[serde(rename="status")]
    pub status: Box<crate::JobStatus>,
}

impl JobStatusChanged {
    pub fn new(status: crate::JobStatus) -> JobStatusChanged {
        JobStatusChanged {
            status: Box::new(status),
        }
    }
}
