// PrintNannyCloudAuthReply represents a PrintNannyCloudAuthReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudAuthReply {
    #[serde(rename="request")]
    pub request: Box<crate::PrintNannyCloudAuthRequest>,
    #[serde(rename="status_code")]
    pub status_code: i32,
    #[serde(rename="msg")]
    pub msg: String,
}

impl PrintNannyCloudAuthReply {
    pub fn new(request: crate::PrintNannyCloudAuthRequest, status_code: i32, msg: String) -> PrintNannyCloudAuthReply {
        PrintNannyCloudAuthReply {
            request: Box::new(request),
            status_code,
            msg,
        }
    }
}
