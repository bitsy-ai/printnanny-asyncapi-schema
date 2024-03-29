// DetectionSettings represents a DetectionSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DetectionSettings {
    #[serde(rename="nats_server_uri")]
    pub nats_server_uri: String,
    #[serde(rename="label_file")]
    pub label_file: String,
    #[serde(rename="model_file")]
    pub model_file: String,
    #[serde(rename="nms_threshold")]
    pub nms_threshold: i32,
    #[serde(rename="tensor_batch_size")]
    pub tensor_batch_size: i32,
    #[serde(rename="tensor_framerate")]
    pub tensor_framerate: i32,
    #[serde(rename="tensor_height")]
    pub tensor_height: i32,
    #[serde(rename="tensor_width")]
    pub tensor_width: i32,
    #[serde(rename="overlay")]
    pub overlay: bool,
    #[serde(rename="graphs")]
    pub graphs: bool,
}

impl DetectionSettings {
    pub fn new(nats_server_uri: String, label_file: String, model_file: String, nms_threshold: i32, tensor_batch_size: i32, tensor_framerate: i32, tensor_height: i32, tensor_width: i32, overlay: bool, graphs: bool) -> DetectionSettings {
        DetectionSettings {
            nats_server_uri,
            label_file,
            model_file,
            nms_threshold,
            tensor_batch_size,
            tensor_framerate,
            tensor_height,
            tensor_width,
            overlay,
            graphs,
        }
    }
}
