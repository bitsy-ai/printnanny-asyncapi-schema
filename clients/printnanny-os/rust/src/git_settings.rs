// GitSettings represents a GitSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GitSettings {
    #[serde(rename="remote")]
    pub remote: String,
    #[serde(rename="email")]
    pub email: String,
    #[serde(rename="name")]
    pub name: String,
    #[serde(rename="default_branch")]
    pub default_branch: String,
}

impl GitSettings {
    pub fn new(remote: String, email: String, name: String, default_branch: String) -> GitSettings {
        GitSettings {
            remote,
            email,
            name,
            default_branch,
        }
    }
}
