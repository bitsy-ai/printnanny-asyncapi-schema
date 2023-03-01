// VideoRecordingPart represents a VideoRecordingPart model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoRecordingPart {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="part")]
    pub part: i32,
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
    pub fn new(id: String, part: i32, size: i64, deleted: bool, sync_start: Option<String>, sync_end: Option<String>, file_name: String, video_recording_id: String) -> VideoRecordingPart {
        VideoRecordingPart {
            id,
            part,
            size,
            deleted,
            sync_start,
            sync_end,
            file_name,
            video_recording_id,
        }
    }
}
