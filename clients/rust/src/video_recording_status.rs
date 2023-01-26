// VideoRecordingStatus represents a VideoRecordingStatus model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VideoRecordingStatus {
    #[serde(rename="pending")]
    Pending,
    #[serde(rename="inprogress")]
    Inprogress,
    #[serde(rename="done")]
    Done,
}
impl Default for VideoRecordingStatus {
    fn default() -> VideoRecordingStatus {
        VideoRecordingStatus::Pending
    }
}