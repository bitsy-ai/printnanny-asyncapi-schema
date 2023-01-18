// PrintNannyCloudSyncReply represents a PrintNannyCloudSyncReply model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudSyncReply {
    #[serde(rename="start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename="end", skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}

impl PrintNannyCloudSyncReply {
    pub fn new(start: Option<i64>, end: Option<i64>) -> PrintNannyCloudSyncReply {
        PrintNannyCloudSyncReply {
            start,
            end,
        }
    }
}
