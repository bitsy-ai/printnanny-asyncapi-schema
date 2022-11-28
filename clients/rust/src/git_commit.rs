// GitCommit represents a GitCommit model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitCommit {
    #[serde(rename="oid")]
    oid: String,
    #[serde(rename="header")]
    header: String,
    #[serde(rename="message")]
    message: String,
    #[serde(rename="ts")]
    ts: i32,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl GitCommit {
    pub fn new(oid: String, header: String, message: String, ts: i32, additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>) -> GitCommit {
        GitCommit {
            oid,
            header,
            message,
            ts,
            additional_properties,
        }
    }
}
