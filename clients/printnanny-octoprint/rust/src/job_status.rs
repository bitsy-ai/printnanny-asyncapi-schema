// JobStatus represents a JobStatus model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct JobStatus {
    #[serde(rename="status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::JobStatus>>,
}

impl JobStatus {
    pub fn new(status: Option<crate::JobStatus>) -> JobStatus {
        JobStatus {
            status: status.map(Box::new),
        }
    }
}
