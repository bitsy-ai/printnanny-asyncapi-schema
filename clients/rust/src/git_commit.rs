// GitCommit represents a GitCommit model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitCommit {
    #[serde(rename="oid", skip_serializing_if = "Option::is_none")]
    oid: Option<String>,
    #[serde(rename="header", skip_serializing_if = "Option::is_none")]
    header: Option<String>,
    #[serde(rename="message", skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    ts: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl GitCommit {
    pub fn new(oid: Option<String>, header: Option<String>, message: Option<String>, ts: Option<i32>, additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>) -> GitCommit {
        GitCommit {
            oid,
            header,
            message,
            ts,
            additional_properties,
        }
    }
}
