use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Setting {
    #[serde(default)]
    ControllerHost: String,
    #[serde(default)]
    ImageHost: String,
    #[serde(default)]
    StreamHost: String,
    #[serde(default)]
    SystemHtml: String,
    #[serde(default)]
    Remark: String,
    #[serde(default)]
    BaseUrl: String,
    #[serde(default)]
    OMUrl: String,
    #[serde(default)]
    SelfPath: String,

    #[serde(default)]
    IsJavBus: bool,
    #[serde(default)]
    IsDb: bool,

    #[serde(default)]
    Tags: Vec<String>,
    #[serde(default)]
    TagsLib: Vec<String>,
    #[serde(default)]
    Dirs: Vec<String>,
    #[serde(default)]
    DirsLib: Vec<String>,
    #[serde(default)]
    ImageTypes: Vec<String>,
    #[serde(default)]
    DocsTypes: Vec<String>,
    #[serde(default)]
    VideoTypes: Vec<String>,
    #[serde(default)]
    Types: Vec<String>,
    #[serde(default)]
    Buttons: Vec<String>,
    #[serde(default)]
    MovieTypes: Vec<String>,
}

impl Setting {
    pub fn new() -> Setting {
        Setting {
            ControllerHost: "".to_string(),
            ImageHost: "".to_string(),
            StreamHost: "".to_string(),
            SystemHtml: "".to_string(),
            Remark: "".to_string(),
            BaseUrl: "".to_string(),
            OMUrl: "".to_string(),
            SelfPath: "".to_string(),
            IsJavBus: false,
            IsDb: false,
            Tags: vec![],
            TagsLib: vec![],
            Dirs: vec![],
            DirsLib: vec![],
            ImageTypes: vec![],
            DocsTypes: vec![],
            VideoTypes: vec![],
            Types: vec![],
            Buttons: vec![],
            MovieTypes: vec![],
        }
    }
    pub fn from(&mut self, req: &Setting) {
        self.ControllerHost = req.ControllerHost.clone();
        self.ImageHost = req.ImageHost.clone();
        self.StreamHost = req.StreamHost.clone();
        self.SystemHtml = req.SystemHtml.clone();
        self.Remark = req.Remark.clone();
        self.BaseUrl = req.BaseUrl.clone();
        self.OMUrl = req.OMUrl.clone();
        self.SelfPath = req.SelfPath.clone();
        self.IsJavBus = req.IsJavBus.clone();
        self.IsDb = req.IsDb.clone();
        self.Tags = req.Tags.clone();
        self.TagsLib = req.TagsLib.clone();
        self.Dirs = req.Dirs.clone();
        self.DirsLib = req.DirsLib.clone();
        self.DocsTypes = req.DocsTypes.clone();
        self.VideoTypes = req.VideoTypes.clone();
        self.Types = req.Types.clone();
        self.Buttons = req.Buttons.clone();
        self.MovieTypes = req.MovieTypes.clone();
    }
}
