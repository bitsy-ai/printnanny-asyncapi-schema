// AnonymousSchema22 represents a AnonymousSchema22 model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AnonymousSchema22 {
    #[serde(rename="request")]
    request: Box<crate::AnonymousSchema23>,
    #[serde(rename="status_code")]
    status_code: i32,
    #[serde(rename="msg")]
    msg: String,
}

impl AnonymousSchema22 {
    pub fn new(request: crate::AnonymousSchema23, status_code: i32, msg: String) -> AnonymousSchema22 {
        AnonymousSchema22 {
        Box::new(request),
        status_code,
        msg,
        }
    }
}
