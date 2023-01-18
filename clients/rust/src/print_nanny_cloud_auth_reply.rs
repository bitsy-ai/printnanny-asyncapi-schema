// PrintNannyCloudAuthReply represents a PrintNannyCloudAuthReply model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudAuthReply {
    #[serde(rename="start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename="end", skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}

impl PrintNannyCloudAuthReply {
    pub fn new(start: Option<i64>, end: Option<i64>) -> PrintNannyCloudAuthReply {
        PrintNannyCloudAuthReply {
            start,
            end,
        }
    }
}
