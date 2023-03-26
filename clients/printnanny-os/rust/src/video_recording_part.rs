// VideoRecordingPart represents a VideoRecordingPart model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoRecordingPart {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="buffer_index")]
    pub buffer_index: i64,
    #[serde(rename="buffer_runningtime")]
    pub buffer_runningtime: i64,
    #[serde(rename="size")]
    pub size: i64,
    #[serde(rename="deleted")]
    pub deleted: bool,
    #[serde(rename="sync_start", skip_serializing_if = "Option::is_none")]
    pub sync_start: Option<String>,
    #[serde(rename="sync_end", skip_serializing_if = "Option::is_none")]
    pub sync_end: Option<String>,
    #[serde(rename="file_name")]
    pub file_name: String,
    #[serde(rename="video_recording_id")]
    pub video_recording_id: String,
}

impl VideoRecordingPart {
    pub fn new(id: String, buffer_index: i64, buffer_runningtime: i64, size: i64, deleted: bool, sync_start: Option<String>, sync_end: Option<String>, file_name: String, video_recording_id: String) -> VideoRecordingPart {
        VideoRecordingPart {
            id,
            buffer_index,
            buffer_runningtime,
            size,
            deleted,
            sync_start,
            sync_end,
            file_name,
            video_recording_id,
        }
    }
}
