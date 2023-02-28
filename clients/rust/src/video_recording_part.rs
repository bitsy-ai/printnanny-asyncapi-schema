// VideoRecordingPart represents a VideoRecordingPart model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoRecordingPart {
    #[serde(rename="id")]
    pub id: String,
    #[serde(rename="part")]
    pub part: i32,
    #[serde(rename="size")]
    pub size: i32,
    #[serde(rename="deleted")]
    pub deleted: bool,
    #[serde(rename="cloud_sync_done")]
    pub cloud_sync_done: bool,
    #[serde(rename="file_name")]
    pub file_name: String,
    #[serde(rename="video_recording_id")]
    pub video_recording_id: String,
}

impl VideoRecordingPart {
    pub fn new(id: String, part: i32, size: i32, deleted: bool, cloud_sync_done: bool, file_name: String, video_recording_id: String) -> VideoRecordingPart {
        VideoRecordingPart {
            id,
            part,
            size,
            deleted,
            cloud_sync_done,
            file_name,
            video_recording_id,
        }
    }
}
