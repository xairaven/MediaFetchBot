use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Display, EnumString, IntoStaticStr)]
pub enum LinkType {
    #[strum(serialize = "tiktok.com")]
    TikTok,
}