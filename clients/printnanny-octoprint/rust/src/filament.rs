// Filament represents a Filament model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Filament {
    #[serde(rename="length")]
    pub length: f64,
    #[serde(rename="volume")]
    pub volume: f64,
    #[serde(rename="toolName")]
    pub tool_name: String,
}

impl Filament {
    pub fn new(length: f64, volume: f64, tool_name: String) -> Filament {
        Filament {
            length,
            volume,
            tool_name,
        }
    }
}
