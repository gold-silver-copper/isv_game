use crate::*;

pub trait GraphicElement {
    fn symbol(&self) -> String;
    fn fg_color(&self) -> RatColor;
    fn bg_color(&self) -> RatColor;

    fn to_graphic_triple(&self) -> GraphicTriple {
        (self.symbol(), self.fg_color(), self.bg_color())
    }
}

// Implementing the trait for Floor
impl GraphicElement for Floor {
    fn symbol(&self) -> String {
        match &self {
            Floor::Gravel(_) => ".".into(),
        }
    }

    fn fg_color(&self) -> RatColor {
        match &self {
            Floor::Gravel(ear) => ear.color(),
        }
    }

    fn bg_color(&self) -> RatColor {
        dim(self.fg_color(), 0.8)
    }
}

// Implementing the trait for Furniture
impl GraphicElement for Furniture {
    fn symbol(&self) -> String {
        match &self {
            Furniture::Wall(_) => "#".into(),
        }
    }

    fn fg_color(&self) -> RatColor {
        match &self {
            Furniture::Wall(sm) => sm.color(),
        }
    }

    fn bg_color(&self) -> RatColor {
        dim(self.fg_color(), 0.8)
    }
}

// Implementing the trait for Roof
impl GraphicElement for Roof {
    fn symbol(&self) -> String {
        match &self {
            Roof::Tegula(_) => "^".into(),
        }
    }

    fn fg_color(&self) -> RatColor {
        match &self {
            Roof::Tegula(sm) => dim(sm.color(), 1.3),
        }
    }

    fn bg_color(&self) -> RatColor {
        match &self {
            Roof::Tegula(sm) => sm.color(),
        }
    }
}

impl GraphicElement for EntityType {
    fn fg_color(&self) -> RatColor {
        match &self {
            _ => RatColor::White,
        }
    }
    fn bg_color(&self) -> RatColor {
        match &self {
            _ => RatColor::Black,
        }
    }
    fn symbol(&self) -> String {
        match &self {
            EntityType::Human(_) => "@".into(),
            EntityType::Item(x) => x.item_symbol(),
            EntityType::Animal(x) => x.item_symbol(),
        }
    }
}
