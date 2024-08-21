use crate::*;

pub trait ToColor {
    fn color(&self) -> RatColor;
}

impl ToColor for Tree {
    fn color(&self) -> RatColor {
        match &self {
            Tree::Glinos => RatColor::Rgb(160, 82, 45),
        }
    }
}

impl ToColor for Metal {
    fn color(&self) -> RatColor {
        match &self {
            Metal::Gold => RatColor::Rgb(10, 32, 95),
        }
    }
}
