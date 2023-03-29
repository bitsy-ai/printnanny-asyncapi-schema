// Job represents a Job model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Job {
    #[serde(rename="file", skip_serializing_if = "Option::is_none")]
    pub file: Option<Box<crate::GcodeFile>>,
    #[serde(rename="estimatedPrintTime", skip_serializing_if = "Option::is_none")]
    pub estimated_print_time: Option<String>,
    #[serde(rename="lastPrintTime", skip_serializing_if = "Option::is_none")]
    pub last_print_time: Option<String>,
    #[serde(rename="filamentLength", skip_serializing_if = "Option::is_none")]
    pub filament_length: Option<String>,
    #[serde(rename="filamentVolume", skip_serializing_if = "Option::is_none")]
    pub filament_volume: Option<String>,
}

impl Job {
    pub fn new(file: Option<crate::GcodeFile>, estimated_print_time: Option<String>, last_print_time: Option<String>, filament_length: Option<String>, filament_volume: Option<String>) -> Job {
        Job {
            file: file.map(Box::new),
            estimated_print_time,
            last_print_time,
            filament_length,
            filament_volume,
        }
    }
}
