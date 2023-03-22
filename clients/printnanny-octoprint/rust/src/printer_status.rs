// PrinterStatus represents a PrinterStatus model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PrinterStatus {
    #[serde(rename="OPEN_SERIAL")]
    OpenSerial,
    #[serde(rename="CONNECTING")]
    Connecting,
    #[serde(rename="OPERATIONAL")]
    Operational,
    #[serde(rename="PRINTING")]
    Printing,
    #[serde(rename="PAUSED")]
    Paused,
    #[serde(rename="CLOSED")]
    Closed,
    #[serde(rename="ERROR")]
    Error,
    #[serde(rename="UNKNOWN")]
    Unknown,
    #[serde(rename="CLOSED_WITH_ERROR")]
    ClosedWithError,
}
impl Default for PrinterStatus {
    fn default() -> PrinterStatus {
        PrinterStatus::OpenSerial
    }
}