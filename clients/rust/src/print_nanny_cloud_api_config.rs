// PrintNannyCloudApiConfig represents a PrintNannyCloudApiConfig model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudApiConfig {
    #[serde(rename="api_base_path")]
    pub api_base_path: String,
    #[serde(rename="api_bearer_access_token", skip_serializing_if = "Option::is_none")]
    pub api_bearer_access_token: Option<String>,
}

impl PrintNannyCloudApiConfig {
    pub fn new(api_base_path: String, api_bearer_access_token: Option<String>) -> PrintNannyCloudApiConfig {
        PrintNannyCloudApiConfig {
            api_base_path,
            api_bearer_access_token,
        }
    }
}
