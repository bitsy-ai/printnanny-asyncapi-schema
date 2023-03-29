// PrinterStatusChanged represents a PrinterStatusChanged model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrinterStatusChanged {
    #[serde(rename="job", skip_serializing_if = "Option::is_none")]
    pub job: Option<Box<crate::Job>>,
    #[serde(rename="status")]
    pub status: Box<crate::PrinterStatus>,
}

impl PrinterStatusChanged {
    pub fn new(job: Option<crate::Job>, status: crate::PrinterStatus) -> PrinterStatusChanged {
        PrinterStatusChanged {
            job: job.map(Box::new),
            status: Box::new(status),
        }
    }
}
