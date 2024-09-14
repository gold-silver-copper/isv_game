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
