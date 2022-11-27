// AnonymousSchema19 represents a AnonymousSchema19 model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnonymousSchema19 {
    #[serde(rename="request")]
    request: serde_json::Value,
    #[serde(rename="unit")]
    unit: Box<crate::SystemdUnit>,
}

impl AnonymousSchema19 {
    pub fn new(request: serde_json::Value, unit: crate::SystemdUnit) -> AnonymousSchema19 {
        AnonymousSchema19 {
        request,
        Box::new(unit),
        }
    }
}
