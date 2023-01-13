// OctoPrintJob represents a OctoPrintJob model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OctoPrintJob {
    #[serde(rename="file", skip_serializing_if = "Option::is_none")]
    pub file: Option<Box<crate::OctoPrintFile>>,
    #[serde(rename="estimatedPrintTime", skip_serializing_if = "Option::is_none")]
    pub estimated_print_time: Option<f64>,
    #[serde(rename="lastPrintType", skip_serializing_if = "Option::is_none")]
    pub last_print_type: Option<f64>,
}

impl OctoPrintJob {
    pub fn new(file: Option<crate::OctoPrintFile>, estimated_print_time: Option<f64>, last_print_type: Option<f64>) -> OctoPrintJob {
        OctoPrintJob {
            file: file.map(Box::new),
            estimated_print_time,
            last_print_type,
        }
    }
}
