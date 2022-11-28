// PrintNannyCloudAuthReply represents a PrintNannyCloudAuthReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudAuthReply {
    #[serde(rename="request")]
    request: Box<crate::PrintNannyCloudAuthRequest>,
    #[serde(rename="status_code")]
    status_code: i32,
    #[serde(rename="msg")]
    msg: String,
}

impl PrintNannyCloudAuthReply {
    pub fn new(request: crate::PrintNannyCloudAuthRequest, status_code: i32, msg: String) -> PrintNannyCloudAuthReply {
        PrintNannyCloudAuthReply {
        Box::new(request),
        status_code,
        msg,
        }
    }
}
