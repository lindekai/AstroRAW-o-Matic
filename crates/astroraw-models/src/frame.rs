use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FrameType {
    #[default]
    Light,
    Dark,
    Flat,
    Bias,
}

impl fmt::Display for FrameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameType::Light => write!(f, "Light Frame"),
            FrameType::Dark => write!(f, "Dark Frame"),
            FrameType::Flat => write!(f, "Flat Frame"),
            FrameType::Bias => write!(f, "Bias Frame"),
        }
    }
}

impl FrameType {
    pub fn fits_imagetyp(&self) -> &'static str {
        match self {
            FrameType::Light => "LIGHT",
            FrameType::Dark => "DARK",
            FrameType::Flat => "FLAT",
            FrameType::Bias => "BIAS",
        }
    }
}
