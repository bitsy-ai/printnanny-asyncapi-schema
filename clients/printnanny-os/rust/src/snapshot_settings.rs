// SnapshotSettings represents a SnapshotSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SnapshotSettings {
    #[serde(rename="path")]
    pub path: String,
    #[serde(rename="enabled")]
    pub enabled: bool,
}

impl SnapshotSettings {
    pub fn new(path: String, enabled: bool) -> SnapshotSettings {
        SnapshotSettings {
            path,
            enabled,
        }
    }
}
