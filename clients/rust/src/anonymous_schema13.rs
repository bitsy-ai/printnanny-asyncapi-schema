// AnonymousSchema13 represents a AnonymousSchema13 model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AnonymousSchema13 {
    #[serde(rename="request")]
    request: Box<crate::AnonymousSchema14>,
    #[serde(rename="changes")]
    changes: Vec<crate::ReservedUnion>,
}

impl AnonymousSchema13 {
    pub fn new(request: crate::AnonymousSchema14, changes: Vec<crate::ReservedUnion>) -> AnonymousSchema13 {
        AnonymousSchema13 {
        Box::new(request),
        changes,
        }
    }
}
