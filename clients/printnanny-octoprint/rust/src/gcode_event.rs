// GcodeEvent represents a GcodeEvent model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum GcodeEvent {
    #[serde(rename="Alert__M300")]
    AlertM300,
    #[serde(rename="Cooling__M245")]
    CoolingM245,
    #[serde(rename="Dwell__G4")]
    DwellG4,
    #[serde(rename="Estop__M112")]
    EstopM112,
    #[serde(rename="FilamentChange__M600")]
    FilamentChangeM600,
    #[serde(rename="FilamentChange__M701")]
    FilamentChangeM701,
    #[serde(rename="FilamentChange__M702")]
    FilamentChangeM702,
    #[serde(rename="Home__G28")]
    HomeG28,
    #[serde(rename="PowerOn__M80")]
    PowerOnM80,
    #[serde(rename="PowerOff__M81")]
    PowerOffM81,
}
impl Default for GcodeEvent {
    fn default() -> GcodeEvent {
        GcodeEvent::AlertM300
    }
}