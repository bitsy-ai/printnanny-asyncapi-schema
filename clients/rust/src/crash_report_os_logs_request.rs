// CrashReportOsLogsRequest represents a CrashReportOsLogsRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CrashReportOsLogsRequest {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="updated_dt")]
    pub updated_dt: String,
}

impl CrashReportOsLogsRequest {
    pub fn new(id: String, updated_dt: String) -> CrashReportOsLogsRequest {
        CrashReportOsLogsRequest {
            id,
            updated_dt,
        }
    }
}
