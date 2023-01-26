// PrintNannyCloudSyncReply represents a PrintNannyCloudSyncReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudSyncReply {
    #[serde(rename="start")]
    pub start: String,
    #[serde(rename="end")]
    pub end: String,
}

impl PrintNannyCloudSyncReply {
    pub fn new(start: String, end: String) -> PrintNannyCloudSyncReply {
        PrintNannyCloudSyncReply {
            start,
            end,
        }
    }
}
