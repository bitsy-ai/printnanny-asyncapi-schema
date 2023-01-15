// SnapshotSettings represents a SnapshotSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotSettings {
    #[serde(rename="path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename="enabled")]
    pub enabled: bool,
}

impl SnapshotSettings {
    pub fn new(path: Option<String>, enabled: bool) -> SnapshotSettings {
        SnapshotSettings {
            path,
            enabled,
        }
    }
}
