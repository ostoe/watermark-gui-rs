

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