// OctoPrintJobProgress represents a OctoPrintJobProgress model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OctoPrintJobProgress {
    #[serde(rename = "completion", skip_serializing_if = "Option::is_none")]
    pub completion: Option<f64>,
    #[serde(rename = "filepos", skip_serializing_if = "Option::is_none")]
    pub filepos: Option<i32>,
    #[serde(rename = "printTime", skip_serializing_if = "Option::is_none")]
    pub print_time: Option<i32>,
    #[serde(rename = "printTimeLeft", skip_serializing_if = "Option::is_none")]
    pub print_time_left: Option<i32>,
    #[serde(
        rename = "printTimeLeftOrigin",
        skip_serializing_if = "Option::is_none"
    )]
    pub print_time_left_origin: Option<String>,
}

impl OctoPrintJobProgress {
    pub fn new(
        completion: Option<f64>,
        filepos: Option<i32>,
        print_time: Option<i32>,
        print_time_left: Option<i32>,
        print_time_left_origin: Option<String>,
    ) -> OctoPrintJobProgress {
        OctoPrintJobProgress {
            completion,
            filepos,
            print_time,
            print_time_left,
            print_time_left_origin,
        }
    }
}
