// OctoPrintServerStatus represents a OctoPrintServerStatus model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OctoPrintServerStatus {
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
    #[serde(rename="status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::OctoPrintServerStatus>>,
}

impl OctoPrintServerStatus {
    pub fn new(additional_properties: Option<serde_json::Value>, status: Option<crate::OctoPrintServerStatus>) -> OctoPrintServerStatus {
        OctoPrintServerStatus {
            additional_properties,
            status: status.map(Box::new),
        }
    }
}
