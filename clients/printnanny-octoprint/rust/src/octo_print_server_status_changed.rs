// OctoPrintServerStatusChanged represents a OctoPrintServerStatusChanged model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OctoPrintServerStatusChanged {
    #[serde(rename="status")]
    pub status: Box<crate::OctoPrintServerStatus>,
}

impl OctoPrintServerStatusChanged {
    pub fn new(status: crate::OctoPrintServerStatus) -> OctoPrintServerStatusChanged {
        OctoPrintServerStatusChanged {
            status: Box::new(status),
        }
    }
}
