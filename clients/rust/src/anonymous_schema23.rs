// AnonymousSchema23 represents a AnonymousSchema23 model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnonymousSchema23 {
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl AnonymousSchema23 {
    pub fn new(additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>) -> AnonymousSchema23 {
        AnonymousSchema23 {
        additional_properties,
        }
    }
}
