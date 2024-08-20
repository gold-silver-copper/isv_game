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

impl ToColor for LiquidType {
    fn color(&self) -> RatColor {
        match &self {
            LiquidType::Water => RatColor::Rgb(35, 137, 218),
           
        }
    }
}

impl ToColor for EarthType {
    fn color(&self) -> RatColor {
        match &self {
            EarthType::Dirt => RatColor::Rgb(145, 118, 83),
       
        }
    }
}
