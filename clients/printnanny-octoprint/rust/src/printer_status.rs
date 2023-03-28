// PrinterStatus represents a PrinterStatus model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PrinterStatus {
    #[serde(rename="CLOSED_WITH_ERROR")]
    ClosedWithError,
    #[serde(rename="CLOSED")]
    Closed,
    #[serde(rename="CONNECTING")]
    Connecting,
    #[serde(rename="DETECT_BAUDRATE")]
    DetectBaudrate,
    #[serde(rename="DETECT_SERIAL")]
    DetectSerial,
    #[serde(rename="ERROR")]
    Error,
    #[serde(rename="NONE")]
    None,
    #[serde(rename="OFFLINE")]
    Offline,
    #[serde(rename="OPEN_SERIAL")]
    OpenSerial,
    #[serde(rename="OPERATIONAL")]
    Operational,
    #[serde(rename="PAUSED")]
    Paused,
    #[serde(rename="PRINTING")]
    Printing,
    #[serde(rename="STARTING")]
    Starting,
    #[serde(rename="TRANSFERING_FILE")]
    TransferingFile,
    #[serde(rename="UNKNOWN")]
    Unknown,
}
impl Default for PrinterStatus {
    fn default() -> PrinterStatus {
        PrinterStatus::ClosedWithError
    }
}