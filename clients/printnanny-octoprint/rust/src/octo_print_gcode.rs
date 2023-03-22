// OctoPrintGcode represents a OctoPrintGcode model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OctoPrintGcode {
    #[serde(rename="gcode")]
    pub gcode: Box<crate::GcodeEvent>,
}

impl OctoPrintGcode {
    pub fn new(gcode: crate::GcodeEvent) -> OctoPrintGcode {
        OctoPrintGcode {
            gcode: Box::new(gcode),
        }
    }
}
