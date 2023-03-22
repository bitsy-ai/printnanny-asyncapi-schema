// PrintNannyCloudAuthReply represents a PrintNannyCloudAuthReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudAuthReply {
    #[serde(rename="status_code")]
    pub status_code: i32,
    #[serde(rename="msg")]
    pub msg: String,
}

impl PrintNannyCloudAuthReply {
    pub fn new(status_code: i32, msg: String) -> PrintNannyCloudAuthReply {
        PrintNannyCloudAuthReply {
            status_code,
            msg,
        }
    }
}
