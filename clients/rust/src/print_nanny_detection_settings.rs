// PrintNannyDetectionSettings represents a PrintNannyDetectionSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PrintNannyDetectionSettings {
    #[serde(rename="nats_server_uri")]
    pub nats_server_uri: String,
    #[serde(rename="label_file")]
    pub label_file: String,
    #[serde(rename="model_file")]
    pub model_file: String,
    #[serde(rename="nms_threshold")]
    pub nms_threshold: i32,
    #[serde(rename="tensor_batch_size", skip_serializing_if = "Option::is_none")]
    pub tensor_batch_size: Option<i32>,
    #[serde(rename="tensor_framerate")]
    pub tensor_framerate: i32,
    #[serde(rename="tensor_height")]
    pub tensor_height: i32,
    #[serde(rename="tensor_width")]
    pub tensor_width: i32,
}

impl PrintNannyDetectionSettings {
    pub fn new(nats_server_uri: String, label_file: String, model_file: String, nms_threshold: i32, tensor_batch_size: Option<i32>, tensor_framerate: i32, tensor_height: i32, tensor_width: i32) -> PrintNannyDetectionSettings {
        PrintNannyDetectionSettings {
            nats_server_uri,
            label_file,
            model_file,
            nms_threshold,
            tensor_batch_size,
            tensor_framerate,
            tensor_height,
            tensor_width,
        }
    }
}
