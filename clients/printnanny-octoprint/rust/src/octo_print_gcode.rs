// OctoPrintGcode represents a OctoPrintGcode model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OctoPrintGcode {
    #[serde(rename="gcode", skip_serializing_if = "Option::is_none")]
    pub gcode: Option<Box<crate::GcodeEvent>>,
}

impl OctoPrintGcode {
    pub fn new(gcode: Option<crate::GcodeEvent>) -> OctoPrintGcode {
        OctoPrintGcode {
            gcode: gcode.map(Box::new),
        }
    }
}
