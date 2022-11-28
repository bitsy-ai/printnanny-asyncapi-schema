// AnonymousSchema12 represents a AnonymousSchema12 model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnonymousSchema12 {
    #[serde(
        rename = "additionalProperties",
        skip_serializing_if = "Option::is_none"
    )]
    additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl AnonymousSchema12 {
    pub fn new(
        additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    ) -> AnonymousSchema12 {
        AnonymousSchema12 {
            additional_properties,
        }
    }
}
