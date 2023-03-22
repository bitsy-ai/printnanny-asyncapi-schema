// CrashReportOsLogsReply represents a CrashReportOsLogsReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CrashReportOsLogsReply {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="updated_dt")]
    pub updated_dt: String,
}

impl CrashReportOsLogsReply {
    pub fn new(id: String, updated_dt: String) -> CrashReportOsLogsReply {
        CrashReportOsLogsReply {
            id,
            updated_dt,
        }
    }
}
