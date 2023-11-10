use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Setting {
    #[serde(default)]
    pub ControllerHost: String,
    #[serde(default)]
    pub ImageHost: String,
    #[serde(default)]
    pub StreamHost: String,
    #[serde(default)]
    pub SystemHtml: String,
    #[serde(default)]
    pub Remark: String,
    #[serde(default)]
    pub BaseUrl: String,
    #[serde(default)]
    pub OMUrl: String,
    #[serde(default)]
    pub SelfPath: String,

    #[serde(default)]
    pub IsJavBus: bool,
    #[serde(default)]
    pub IsDb: bool,

    #[serde(default)]
    pub Tags: Vec<String>,
    #[serde(default)]
    pub TagsLib: Vec<String>,
    #[serde(default)]
    pub Dirs: Vec<String>,
    #[serde(default)]
    pub DirsLib: Vec<String>,
    #[serde(default)]
    pub ImageTypes: Vec<String>,
    #[serde(default)]
    pub DocsTypes: Vec<String>,
    #[serde(default)]
    pub VideoTypes: Vec<String>,
    #[serde(default)]
    pub Types: Vec<String>,
    #[serde(default)]
    pub Buttons: Vec<String>,
    #[serde(default)]
    pub MovieTypes: Vec<String>,
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
