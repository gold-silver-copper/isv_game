use crate::*;

pub trait ToColor {
    fn color(&self) -> RatColor;
}

impl ToColor for Mineral {
    fn color(&self) -> RatColor {
        RatColor::Magenta
    }
}

impl ToColor for Metal {
    fn color(&self) -> RatColor {
        RatColor::Magenta
    }
}

impl ToColor for SolidMaterial {
    fn color(&self) -> RatColor {
        match &self {
            Self::Wood(inner) => inner.color(),
            Self::Stone(inner) => inner.color(),
            Self::Metal(inner) => inner.color(),
            _ => todo!("IMPLEMENT COLORS FOR MATERIAL"),
        }
    }
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
            LiquidType::Lava => RatColor::Rgb(131, 40, 40),
            LiquidType::Beer => RatColor::Rgb(35, 37, 118),
        }
    }
}

impl ToColor for EarthType {
    fn color(&self) -> RatColor {
        match &self {
            EarthType::Dirt => RatColor::Rgb(145, 118, 83),
            EarthType::Clay => RatColor::Rgb(214, 156, 44),
            EarthType::Sand => RatColor::Rgb(215, 216, 41),
        }
    }
}
