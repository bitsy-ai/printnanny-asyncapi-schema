// SystemdUnitLoadState represents a SystemdUnitLoadState model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SystemdUnitLoadState {
    #[serde(rename="loaded")]
    Loaded,
    #[serde(rename="error")]
    Error,
    #[serde(rename="masked")]
    Masked,
}
impl Default for SystemdUnitLoadState {
    fn default() -> SystemdUnitLoadState {
        SystemdUnitLoadState::Loaded
    }
}