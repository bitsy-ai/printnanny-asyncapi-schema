// VideoSourceType represents a VideoSourceType model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VideoSourceType {
    #[serde(rename="csi")]
    Csi,
    #[serde(rename="usb")]
    Usb,
    #[serde(rename="uri")]
    Uri,
    #[serde(rename="file")]
    File,
}
impl Default for VideoSourceType {
    fn default() -> VideoSourceType {
        VideoSourceType::Csi
    }
}