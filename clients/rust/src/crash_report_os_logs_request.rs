// CrashReportOsLogsRequest represents a CrashReportOsLogsRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CrashReportOsLogsRequest {
    #[serde(rename="id")]
    pub id: String,
}

impl CrashReportOsLogsRequest {
    pub fn new(id: String) -> CrashReportOsLogsRequest {
        CrashReportOsLogsRequest {
            id,
        }
    }
}
