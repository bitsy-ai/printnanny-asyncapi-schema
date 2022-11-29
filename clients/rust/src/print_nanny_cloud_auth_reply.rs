// PrintNannyCloudAuthReply represents a PrintNannyCloudAuthReply model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrintNannyCloudAuthReply {
    #[serde(rename="request")]
    pub request: serde_json::Value,
    #[serde(rename="status_code")]
    pub status_code: i32,
    #[serde(rename="msg")]
    pub msg: String,
}

impl PrintNannyCloudAuthReply {
    pub fn new(request: serde_json::Value, status_code: i32, msg: String) -> PrintNannyCloudAuthReply {
        PrintNannyCloudAuthReply {
            request,
            status_code,
            msg,
        }
    }
}
