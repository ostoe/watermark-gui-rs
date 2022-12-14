use std::path::PathBuf;

use serde::{Serialize, Deserialize};



// pub struct BannerUnit{
//     pub banner_type: BannerType,
//     pub size: (u32, u32),
//     pub position: f32, // % 
//     // time_str: String,
// }
// 利用 宏实现 enum display
pub enum  BannerType{
    Logo,
    Text(String),
}


pub enum UserOperation {
    Cancel,
    Pause,
    Update(UserSetting),
    DirPath(String),
    ImagePath(String),
    Init(PathBuf),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Notification {
    Single(String, String),
    Complated,
    Error(String),
    SkipFile(String),
}
#[derive(Serialize, Deserialize)]
pub struct ImagesPathFromFront {
    pub count: u32,
    pub image_paths: Vec<String>,
}

pub enum UserSetting {
    OutputDir(String),
    Qulity(u8),
    AutoUseBrand(bool, String),
    Font(String),
    FileNamePattern([String;3]),
    Style(WaterMarkStyle)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WaterMarkStyle {
     pub watermark_wh_ratio: f32,
     pub watermark_text_h_scale: f32,
     pub logo_ratio: f32,
    //  pub logo_spacing_ratio: f32,
     pub position_ratio: f32,
     pub split_line_spacing: u32,
    //  pub font_path: String,
     pub font_scale: f32,
     pub datetime_posi_percent: f32,
     pub camera_color: String,
     pub focus_color: String,
     pub datetime_color: String,
     pub  splite_line_color : String,
     pub banner_bg_color : String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserSettingsJson {
    pub output_dir: String,
    pub qulity: u8,
    pub auto_user_brand: bool,
    pub brand: String,
    pub filename_pattern: [String; 3],
}