// AnonymousSchema17 represents a AnonymousSchema17 model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AnonymousSchema17 {
    #[serde(rename="name")]
    name: String,
}

impl AnonymousSchema17 {
    pub fn new(name: String) -> AnonymousSchema17 {
        AnonymousSchema17 {
        name,
        }
    }
}
