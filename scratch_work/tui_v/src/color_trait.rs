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
            Metal::Gold => RatColor::Rgb(100, 32, 95),
        }
    }
}

impl ToColor for Material {
    fn color(&self) -> RatColor {
        match &self {
            Material::Metal(met) => met.color(),
            Material::Wood(wod) => wod.color(),
        }
    }
}