// CameraSourceType represents a CameraSourceType model.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CameraSourceType {
    #[serde(rename="csi")]
    Csi,
    #[serde(rename="usb")]
    Usb,
}
impl Default for CameraSourceType {
    fn default() -> CameraSourceType {
        CameraSourceType::Csi
    }
}