// JobProgress represents a JobProgress model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct JobProgress {
    #[serde(rename="completion", skip_serializing_if = "Option::is_none")]
    pub completion: Option<f64>,
    #[serde(rename="filepos", skip_serializing_if = "Option::is_none")]
    pub filepos: Option<i32>,
    #[serde(rename="print_time", skip_serializing_if = "Option::is_none")]
    pub print_time: Option<i32>,
    #[serde(rename="print_time_left", skip_serializing_if = "Option::is_none")]
    pub print_time_left: Option<i32>,
    #[serde(rename="print_time_left_origin", skip_serializing_if = "Option::is_none")]
    pub print_time_left_origin: Option<String>,
}

impl JobProgress {
    pub fn new(completion: Option<f64>, filepos: Option<i32>, print_time: Option<i32>, print_time_left: Option<i32>, print_time_left_origin: Option<String>) -> JobProgress {
        JobProgress {
            completion,
            filepos,
            print_time,
            print_time_left,
            print_time_left_origin,
        }
    }
}
