// ReservedUnion represents a union of types: String, serde_json::Value
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReservedUnion {
    #[serde(rename="AnonymousSchema160")]
    AnonymousSchema160(String),
    #[serde(rename="Undefined1")]
    Undefined1(serde_json::Value),
}

