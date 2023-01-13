// OctoPrintFile represents a OctoPrintFile model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OctoPrintFile {
    #[serde(rename="name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename="display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename="path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename="origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

impl OctoPrintFile {
    pub fn new(name: Option<String>, display: Option<String>, path: Option<String>, origin: Option<String>) -> OctoPrintFile {
        OctoPrintFile {
            name,
            display,
            path,
            origin,
        }
    }
}
