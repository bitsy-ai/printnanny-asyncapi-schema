// OctoPrintCurrentJob represents a OctoPrintCurrentJob model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OctoPrintCurrentJob {
    #[serde(rename = "job")]
    pub job: Box<crate::OctoPrintJob>,
    #[serde(rename = "progress")]
    pub progress: Box<crate::OctoPrintJobProgress>,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl OctoPrintCurrentJob {
    pub fn new(
        job: crate::OctoPrintJob,
        progress: crate::OctoPrintJobProgress,
        state: String,
        error: Option<String>,
    ) -> OctoPrintCurrentJob {
        OctoPrintCurrentJob {
            job: Box::new(job),
            progress: Box::new(progress),
            state,
            error,
        }
    }
}
