use crate::*;

pub trait GraphicElement {
    fn symbol(&self) -> &'static str;
    fn fg_color(&self) -> RatColor;
    fn bg_color(&self) -> RatColor;

    fn to_graphic_triple(&self) -> GraphicTriple {
        (self.symbol(), self.fg_color(), self.bg_color())
    }
}

// Implementing the trait for Floor
impl GraphicElement for Floor {
    fn symbol(&self) -> &'static str {
        match &self {
            Floor::Liquid(_) => "~",
            Floor::Earth(_) => ".",
        }
    }

    fn fg_color(&self) -> RatColor {
        dim(self.bg_color(), 1.2)
    }

    fn bg_color(&self) -> RatColor {
        match &self {
            Floor::Liquid(liq) => liq.color(),
            Floor::Earth(ear) => ear.color(),
        }
    }
}

// Implementing the trait for Furniture
impl<T: ColoredMaterial> GraphicElement for Furniture<T> {
    fn symbol(&self) -> &'static str {
        match &self {
            Furniture::Wall(_) => "#",
         
        }
    }

    fn fg_color(&self) -> RatColor {
        match &self {
            Furniture::Wall(sm) => sm.color(),
     
        }
    }

    fn bg_color(&self) -> RatColor {
        self.fg_color()
    }
}

// Implementing the trait for Roof
impl GraphicElement for Roof {
    fn symbol(&self) -> &'static str {
        match &self {
            Roof::Tegula(_) => "^",
      
        }
    }

    fn fg_color(&self) -> RatColor {
        match &self {
            Roof::Tegula(sm) => dim(sm.mat_color(), 1.3),
        
        }
    }

    fn bg_color(&self) -> RatColor {
        match &self {
            Roof::Tegula(sm) => sm.mat_color(),
       
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
    fn symbol(&self) -> &'static str {
        match &self {
            _ => "@",
        }
    }
}