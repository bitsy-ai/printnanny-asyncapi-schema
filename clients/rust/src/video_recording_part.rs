// VideoRecordingPart represents a VideoRecordingPart model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoRecordingPart {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="buffer_index", skip_serializing_if = "Option::is_none")]
    pub buffer_index: Option<i32>,
    #[serde(rename="buffer_ts")]
    pub buffer_ts: i64,
    #[serde(rename="buffer_streamtime")]
    pub buffer_streamtime: i64,
    #[serde(rename="buffer_runningtime")]
    pub buffer_runningtime: i64,
    #[serde(rename="buffer_duration")]
    pub buffer_duration: i64,
    #[serde(rename="buffer_offset")]
    pub buffer_offset: i64,
    #[serde(rename="buffer_offset_end")]
    pub buffer_offset_end: i64,
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
    pub fn new(id: String, buffer_index: Option<i32>, buffer_ts: i64, buffer_streamtime: i64, buffer_runningtime: i64, buffer_duration: i64, buffer_offset: i64, buffer_offset_end: i64, size: i64, deleted: bool, sync_start: Option<String>, sync_end: Option<String>, file_name: String, video_recording_id: String) -> VideoRecordingPart {
        VideoRecordingPart {
            id,
            buffer_index,
            buffer_ts,
            buffer_streamtime,
            buffer_runningtime,
            buffer_duration,
            buffer_offset,
            buffer_offset_end,
            size,
            deleted,
            sync_start,
            sync_end,
            file_name,
            video_recording_id,
        }
    }
}
