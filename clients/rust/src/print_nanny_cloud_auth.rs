// PrintNannyCloudAuth represents a PrintNannyCloudAuth model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyCloudAuth {
    #[serde(rename="email")]
    email: String,
    #[serde(rename="api_token")]
    api_token: String,
    #[serde(rename="api_url")]
    api_url: String,
}

impl PrintNannyCloudAuth {
    pub fn new(email: String, api_token: String, api_url: String) -> PrintNannyCloudAuth {
        PrintNannyCloudAuth {
        email,
        api_token,
        api_url,
        }
    }
}
