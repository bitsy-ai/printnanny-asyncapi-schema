// ActionReply represents a ActionReply model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ActionReply {
    #[serde(rename="request")]
    request: Box<crate::AnonymousSchema17>,
    #[serde(rename="status_code")]
    status_code: i32,
    #[serde(rename="msg")]
    msg: String,
}

impl ActionReply {
    pub fn new(request: crate::AnonymousSchema17, status_code: i32, msg: String) -> ActionReply {
        ActionReply {
        Box::new(request),
        status_code,
        msg,
        }
    }
}
