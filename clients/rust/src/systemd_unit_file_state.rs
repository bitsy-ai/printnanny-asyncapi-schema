// SystemdUnitFileState represents a SystemdUnitFileState model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SystemdUnitFileState {
    #[serde(rename="enabled")]
    Enabled,
    #[serde(rename="enabled-runtime")]
    EnabledMinusRuntime,
    #[serde(rename="linked")]
    Linked,
    #[serde(rename="linked-runtime")]
    LinkedMinusRuntime,
    #[serde(rename="masked")]
    Masked,
    #[serde(rename="masked-runtime")]
    MaskedMinusRuntime,
    #[serde(rename="static")]
    ReservedStatic,
    #[serde(rename="disabled")]
    Disabled,
    #[serde(rename="invalid")]
    Invalid,
}
impl Default for SystemdUnitFileState {
    fn default() -> SystemdUnitFileState {
        SystemdUnitFileState::Enabled
    }
}