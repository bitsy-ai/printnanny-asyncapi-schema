// ReservedUnion represents a union of types: GitCommit0, serde_json::Value
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReservedUnion {
    #[serde(rename="GitCommit0")]
    GitCommit0(crate::GitCommit),
    #[serde(rename="Undefined1")]
    Undefined1(serde_json::Value),
}

