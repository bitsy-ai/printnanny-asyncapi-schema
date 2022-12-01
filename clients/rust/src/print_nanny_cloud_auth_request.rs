// PrintNannyCloudAuthRequest represents a PrintNannyCloudAuthRequest model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudAuthRequest {
    #[serde(rename="email")]
    pub email: String,
    #[serde(rename="api_token")]
    pub api_token: String,
    #[serde(rename="api_url")]
    pub api_url: String,
}

impl PrintNannyCloudAuthRequest {
    pub fn new(email: String, api_token: String, api_url: String) -> PrintNannyCloudAuthRequest {
        PrintNannyCloudAuthRequest {
            email,
            api_token,
            api_url,
        }
    }
}
