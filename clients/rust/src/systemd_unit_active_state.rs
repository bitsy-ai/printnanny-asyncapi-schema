// SystemdUnitActiveState represents a SystemdUnitActiveState model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SystemdUnitActiveState {
    #[serde(rename="active")]
    Active,
    #[serde(rename="activating")]
    Activating,
    #[serde(rename="deactivating")]
    Deactivating,
    #[serde(rename="inactive")]
    Inactive,
    #[serde(rename="reloading")]
    Reloading,
    #[serde(rename="loaded")]
    Loaded,
}
impl Default for SystemdUnitActiveState {
    fn default() -> SystemdUnitActiveState {
        SystemdUnitActiveState::Active
    }
}