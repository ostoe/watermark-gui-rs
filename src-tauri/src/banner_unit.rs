use std::path::PathBuf;

use serde::{Serialize, Deserialize};



pub struct BannerUnit{
    banner_type: BannerType,
    size: (u32, u32),
    position: f32, // % 
    // time_str: String,
}
// 利用 宏实现 enum display
pub enum  BannerType{
    Logo,
    Text(String),
}


pub enum UserOperation {
    Cancel,
    Pause,
    Update(String, String),
    DirPath(String),
    ImagePath(String),
    Init(PathBuf),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Notification {
    Single(String),
    Complated,
    Error(String),
    SkipFile(String),
}
#[derive(Serialize, Deserialize)]
pub struct ImagesPathFromFront {
    pub count: u32,
    pub image_paths: Vec<String>,
}

pub enum UserSettings {
    OutputDir(String),
    Qulity(u8),
    AutoUseBrand(bool),

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserSettingsJson {
    pub output_dir: String,
    pub qulity: u8,
    pub auto_user_brand: bool,
}