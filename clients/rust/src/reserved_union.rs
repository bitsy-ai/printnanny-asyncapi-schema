// ReservedUnion represents a union of types: SystemdUnitChange0, serde_json::Value
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReservedUnion {
    #[serde(rename="SystemdUnitChange0")]
    SystemdUnitChange0(crate::SystemdUnitChange),
    #[serde(rename="Undefined1")]
    Undefined1(serde_json::Value),
}

