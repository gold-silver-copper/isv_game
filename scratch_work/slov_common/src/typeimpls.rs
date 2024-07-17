use crate::*;

impl Floor {
    pub fn fg_color(&self) -> RatColor {
        dim(self.bg_color(), 1.2)
    }
    pub fn bg_color(&self) -> RatColor {
        match &self {
            Floor::Liquid(liq) => liq.color(),
            Floor::Earth(ear) => ear.color(),
        }
    }
    pub fn symbol(&self) -> &'static str {
        match &self {
            Floor::Liquid(_) => "~",
            Floor::Earth(_) => ".",
        }
    }

    pub fn to_graphic_triple(&self) -> GraphicTriple {
        (self.symbol(), self.fg_color(), self.bg_color())
    }
}

impl Furniture {
    pub fn to_color(&self) -> RatColor {
        match &self {
            Furniture::Wall(sm) => sm.to_color(),
            Furniture::Door(sm) => sm.to_color(),
            Furniture::Trinket => RatColor::White,
        }
    }
    pub fn symbol(&self) -> &'static str {
        match &self {
            Furniture::Wall(sm) => "#",
            Furniture::Door(sm) => "+",
            Furniture::Trinket => " ",
        }
    }

    pub fn blocks_movement(&self) -> bool {
        match &self {
            Furniture::Wall(_) => true,
            _ => false,
        }
    }
    pub fn blocks_vision(&self) -> bool {
        match &self {
            Furniture::Wall(_) => true,
            _ => false,
        }
    }
}

impl Roof {
    pub fn to_fg_color(&self) -> RatColor {
        match &self {
            Roof::Tegula(sm) => dim(sm.to_color(), 1.3),
            Roof::Imbrex(sm) => dim(sm.to_color(), 1.3),
        }
    }
    pub fn to_bg_color(&self) -> RatColor {
        match &self {
            Roof::Tegula(sm) => sm.to_color(),
            Roof::Imbrex(sm) => sm.to_color(),
        }
    }
    pub fn symbol(&self) -> &'static str {
        match &self {
            Roof::Tegula(_) => "^",
            Roof::Imbrex(_) => "=",
        }
    }
}

impl EntityType {
    pub fn to_fg_color(&self) -> RatColor {
        match &self {
            EntityType::Humanus => RatColor::White,
        }
    }
    pub fn to_bg_color(&self) -> RatColor {
        match &self {
            EntityType::Humanus => RatColor::Black,
        }
    }
    pub fn symbol(&self) -> &'static str {
        match &self {
            EntityType::Humanus => "@",
        }
    }
    pub fn to_graphic_triple(&self) -> GraphicTriple {
        (self.symbol(), self.to_fg_color(), self.to_bg_color())
    }
    pub fn blocks_movement(&self) -> bool {
        match &self {
            EntityType::Humanus => true,
            _ => false,
        }
    }
}

impl Mineral {
    pub fn color(&self) -> RatColor {
        match &self {
            _ => RatColor::Magenta,
        }
    }
}
impl Metal {
    pub fn color(&self) -> RatColor {
        match &self {
            _ => RatColor::Magenta,
        }
    }
}

impl SolidMaterial {
    pub fn to_color(&self) -> RatColor {
        match &self {
            Self::Wood(inner) => inner.color(),
            Self::Stone(inner) => inner.color(),
            Self::Metal(inner) => inner.color(),
            _ => todo!("IMPLEMENT COLORS FOR MATERIAL"),
        }
    }
}

impl Tree {
    pub fn color(&self) -> RatColor {
        match &self {
            Tree::Glinos => RatColor::Rgb(160, 82, 45),
        }
    }
}

impl LiquidType {
    pub fn color(&self) -> RatColor {
        match &self {
            LiquidType::Water => RatColor::Rgb(35, 137, 218),
            LiquidType::Lava => RatColor::Rgb(131, 40, 40),
            LiquidType::Beer => RatColor::Rgb(35, 37, 118),
        }
    }
}

impl EarthType {
    pub fn color(&self) -> RatColor {
        match &self {
            EarthType::Dirt => RatColor::Rgb(145, 118, 83),
            EarthType::Clay => RatColor::Rgb(214, 156, 44),
            EarthType::Sand => RatColor::Rgb(215, 216, 41),
        }
    }
}
