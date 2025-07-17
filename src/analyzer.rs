pub struct StaadBackgroundAnalyzer {
    staad_process: Option<Child>,
    staad_app: Option<IDispatch>,
    geometry: Option<IDispatch>,
    analysis: Option<IDispatch>,
    post: Option<IDispatch>,
    property: Option<IDispatch>,
    load: Option<IDispatch>,
    staad_path: String,
}

impl StaadBackgroundAnalyzer {
    pub fn new(staad_path: &str) -> Self {
        StaadBackgroundAnalyzer {
            staad_process: None,
            staad_app: None,
            geometry: None,
            analysis: None,
            post: None,
            property: None,
            load: None,
            staad_path: staad_path.to_string(),
        }
    }
}
