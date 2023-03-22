// JobStatus represents a JobStatus model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JobStatus {
    #[serde(rename="PrintStarted")]
    PrintStarted,
    #[serde(rename="PrintFailed")]
    PrintFailed,
    #[serde(rename="PrintDone")]
    PrintDone,
    #[serde(rename="PrintCancelling")]
    PrintCancelling,
    #[serde(rename="PrintCanelled")]
    PrintCanelled,
    #[serde(rename="PrintPaused")]
    PrintPaused,
    #[serde(rename="PrintResumed")]
    PrintResumed,
}
impl Default for JobStatus {
    fn default() -> JobStatus {
        JobStatus::PrintStarted
    }
}