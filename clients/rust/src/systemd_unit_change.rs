// SystemdUnitChange represents a SystemdUnitChange model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemdUnitChange {
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl SystemdUnitChange {
    pub fn new(additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>) -> SystemdUnitChange {
        SystemdUnitChange {
        additional_properties,
        }
    }
}
