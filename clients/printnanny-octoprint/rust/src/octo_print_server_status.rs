// OctoPrintServerStatus represents a OctoPrintServerStatus model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OctoPrintServerStatus {
    #[serde(rename="Startup")]
    Startup,
    #[serde(rename="Shutdown")]
    Shutdown,
}
impl Default for OctoPrintServerStatus {
    fn default() -> OctoPrintServerStatus {
        OctoPrintServerStatus::Startup
    }
}