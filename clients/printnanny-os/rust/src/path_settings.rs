// PathSettings represents a PathSettings model.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PathSettings {
    #[serde(rename="state_dir")]
    pub state_dir: String,
    #[serde(rename="settings_dir")]
    pub settings_dir: String,
    #[serde(rename="log_dir")]
    pub log_dir: String,
    #[serde(rename="run_dir")]
    pub run_dir: String,
    #[serde(rename="issue_txt")]
    pub issue_txt: String,
    #[serde(rename="os_release")]
    pub os_release: String,
}

impl PathSettings {
    pub fn new(state_dir: String, settings_dir: String, log_dir: String, run_dir: String, issue_txt: String, os_release: String) -> PathSettings {
        PathSettings {
            state_dir,
            settings_dir,
            log_dir,
            run_dir,
            issue_txt,
            os_release,
        }
    }
}
