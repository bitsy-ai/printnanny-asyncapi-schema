// SystemdUnitChangeState represents a SystemdUnitChangeState model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SystemdUnitChangeState {
    #[serde(rename="symlink")]
    Symlink,
    #[serde(rename="unlink")]
    Unlink,
}
impl Default for SystemdUnitChangeState {
    fn default() -> SystemdUnitChangeState {
        SystemdUnitChangeState::Symlink
    }
}