// AnonymousSchema14 represents a AnonymousSchema14 model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnonymousSchema14 {
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl AnonymousSchema14 {
    pub fn new(additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>) -> AnonymousSchema14 {
        AnonymousSchema14 {
        additional_properties,
        }
    }
}
