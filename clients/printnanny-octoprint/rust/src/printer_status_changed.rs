// PrinterStatusChanged represents a PrinterStatusChanged model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrinterStatusChanged {
    #[serde(rename="status")]
    pub status: Box<crate::PrinterStatus>,
}

impl PrinterStatusChanged {
    pub fn new(status: crate::PrinterStatus) -> PrinterStatusChanged {
        PrinterStatusChanged {
            status: Box::new(status),
        }
    }
}
