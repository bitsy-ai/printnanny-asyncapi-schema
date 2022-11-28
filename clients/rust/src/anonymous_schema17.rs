// AnonymousSchema17 represents a AnonymousSchema17 model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnonymousSchema17 {
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl AnonymousSchema17 {
    pub fn new(additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>) -> AnonymousSchema17 {
        AnonymousSchema17 {
        additional_properties,
        }
    }
}
