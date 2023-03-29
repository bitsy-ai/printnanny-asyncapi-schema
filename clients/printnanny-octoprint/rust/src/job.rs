// Job represents a Job model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Job {
    #[serde(rename="file")]
    pub file: Box<crate::GcodeFile>,
    #[serde(rename="averagePrintTime", skip_serializing_if = "Option::is_none")]
    pub average_print_time: Option<f64>,
    #[serde(rename="estimatedPrintTime")]
    pub estimated_print_time: f64,
    #[serde(rename="lastPrintTime")]
    pub last_print_time: f64,
    #[serde(rename="filaments", skip_serializing_if = "Option::is_none")]
    pub filaments: Option<Vec<crate::Filament>>,
}

impl Job {
    pub fn new(file: crate::GcodeFile, average_print_time: Option<f64>, estimated_print_time: f64, last_print_time: f64, filaments: Option<Vec<crate::Filament>>) -> Job {
        Job {
            file: Box::new(file),
            average_print_time,
            estimated_print_time,
            last_print_time,
            filaments,
        }
    }
}
