// GitCommit represents a GitCommit model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GitCommit {
    #[serde(rename="oid")]
    pub oid: String,
    #[serde(rename="header")]
    pub header: String,
    #[serde(rename="message")]
    pub message: String,
    #[serde(rename="ts")]
    pub ts: i32,
}

impl GitCommit {
    pub fn new(oid: String, header: String, message: String, ts: i32) -> GitCommit {
        GitCommit {
            oid,
            header,
            message,
            ts,
        }
    }
}
