// AnonymousSchema10 represents a AnonymousSchema10 model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AnonymousSchema10 {
    #[serde(rename="files")]
    files: Vec<crate::ReservedUnion>,
}

impl AnonymousSchema10 {
    pub fn new(files: Vec<crate::ReservedUnion>) -> AnonymousSchema10 {
        AnonymousSchema10 {
        files,
        }
    }
}
