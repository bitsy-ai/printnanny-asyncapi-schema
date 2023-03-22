// GcodeFile represents a GcodeFile model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GcodeFile {
    #[serde(rename="name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename="display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename="path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename="origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

impl GcodeFile {
    pub fn new(name: Option<String>, display: Option<String>, path: Option<String>, origin: Option<String>) -> GcodeFile {
        GcodeFile {
            name,
            display,
            path,
            origin,
        }
    }
}
