// JobStatus represents a JobStatus model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct JobStatus {
    #[serde(rename="status")]
    pub status: Box<crate::JobStatus>,
}

impl JobStatus {
    pub fn new(status: crate::JobStatus) -> JobStatus {
        JobStatus {
            status: Box::new(status),
        }
    }
}
