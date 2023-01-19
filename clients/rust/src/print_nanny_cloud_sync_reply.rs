// PrintNannyCloudSyncReply represents a PrintNannyCloudSyncReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudSyncReply {
    #[serde(rename="start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename="end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

impl PrintNannyCloudSyncReply {
    pub fn new(start: Option<String>, end: Option<String>) -> PrintNannyCloudSyncReply {
        PrintNannyCloudSyncReply {
            start,
            end,
        }
    }
}
