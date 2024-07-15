use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Voxel {
    pub floor: Option<Floor>,
    pub roof: Option<Roof>,
    pub furniture: Option<Furniture>,
    pub voxel_pos: MyPoint,
}

impl Voxel {
    pub fn to_graphic(&self, with_roof: bool) -> GraphicTriple {

        let mut floor =  match &self.floor {
            Some(fl) => fl.to_graphic_triple(),
            None => (" ",RatColor::Black,RatColor::Black)
        };
        
 



        let mut plus_furn: GraphicTriple = match &self.furniture {
            Some(furn) => (furn.symbol(), furn.to_color(), floor.2.clone()),
            None => floor,
        };

        if with_roof {
            let mut plus_roof: GraphicTriple = match &self.roof {
                Some(roof) => (roof.symbol(), roof.to_fg_color(), roof.to_bg_color()),
                None => plus_furn,
            };

            plus_roof
        } else {
            plus_furn
        }
    }
}

impl RTreeObject for Voxel {
    type Envelope = AABB<(i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.voxel_pos.0, self.voxel_pos.1))
    }
}

impl PointDistance for Voxel {
    fn distance_2(&self, point: &(i64, i64)) -> i64 {
        self.voxel_pos.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64)) -> bool {
        self.voxel_pos.contains_point(point)
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Floor {
    Liquid(LiquidType),
    Earth(EarthType),
}

impl Floor {
    pub fn fg_color(&self) -> RatColor {
        dim(self.bg_color(), 1.2)
    }
    pub fn bg_color(&self) -> RatColor {
        match self {
            Floor::Liquid(liq) => liq.color(),
            Floor::Earth(ear) => ear.color(),
        }
    }
    pub fn symbol(&self) ->  &'static str {
        match self {
            Floor::Liquid(_) => "~",
            Floor::Earth(_) => ".",
        }
    }

    pub fn to_graphic_triple(&self) -> GraphicTriple {
        (self.symbol(), self.fg_color(), self.bg_color())
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum LiquidType {
    Water,
    Lava,
    Beer,
}

pub fn dim(color: RatColor, factor: f32) -> RatColor {
    match color {
        RatColor::Rgb(r, g, b) => RatColor::Rgb(
            ((r as f32 * factor).clamp(0.0, 127.0)) as u8,
            ((g as f32 * factor).clamp(0.0, 127.0)) as u8,
            ((b as f32 * factor).clamp(0.0, 127.0)) as u8,
        ),
        _ => RatColor::Gray,
    }
}

impl LiquidType {
    pub fn color(&self) -> RatColor {
        match self {
            LiquidType::Water => RatColor::Rgb(35, 137, 218),
            LiquidType::Lava => RatColor::Rgb(131,40,40),
            LiquidType::Beer => RatColor::Rgb(35, 37, 118),
        }
    }
}

impl EarthType {
    pub fn color(&self) -> RatColor {
        match self {
            EarthType::Dirt => RatColor::Rgb(145, 118, 83),
            EarthType::Clay => RatColor::Rgb(214,156,44),
            EarthType::Sand => RatColor::Rgb(215,216,41),
        }
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum EarthType {
    Dirt,
    Clay,
    Sand,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Roof {
    Tegula(SolidMaterial),
    Imbrex(SolidMaterial),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SolidMaterial {
    Wood(Tree),
    Stone(Mineral),
    Metal(Metal),
    Terracotta(EarthType),
}

impl SolidMaterial {
    pub fn to_color(&self) -> RatColor {
        match self {
            Self::Wood(inner) => inner.color(),
            Self::Stone(inner) => inner.color(),
            Self::Metal(inner) => inner.color(),
            _ => todo!("IMPLEMENT COLORS FOR MATERIAL"),
        }
    }
}

impl Tree {
    pub fn color(&self) -> RatColor {
        match self {
            Tree::Glinos => RatColor::Rgb(51, 34, 17),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    Glinos, //Maple Tree
}

impl Mineral {
    pub fn color(&self) -> RatColor {
        match self {
            _ => RatColor::Magenta,
        }
    }
}
impl Metal {
    pub fn color(&self) -> RatColor {
        match self {
            _ => RatColor::Magenta,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mineral {
    Iaspis, //Gold
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metal {
    Glinos, //Maple Tree
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Furniture {
    Wall(SolidMaterial),
    Door(SolidMaterial),
    Trinket,
}

impl Furniture {
    pub fn to_color(&self) -> RatColor {
        match &self {
            Furniture::Wall(sm) => sm.to_color(),
            Furniture::Door(sm) => sm.to_color(),
            Furniture::Trinket => RatColor::White,
        }
    }
    pub fn symbol(&self) ->  &'static str {
        match &self {
            Furniture::Wall(sm) => "#",
            Furniture::Door(sm) => "+",
            Furniture::Trinket => " ",
        }
    }
}

impl Roof {
    pub fn to_fg_color(&self) -> RatColor {
        match &self {
            Roof::Tegula(sm) => dim(sm.to_color(),1.3),
            Roof::Imbrex(sm) => dim(sm.to_color(),1.3),
        }
    }
    pub fn to_bg_color(&self) -> RatColor {
        match &self {
            Roof::Tegula(sm) => sm.to_color(),
            Roof::Imbrex(sm) => sm.to_color(),
        }
    }
    pub fn symbol(&self) -> &'static str {
        match self {
            Roof::Tegula(_) => "^",
            Roof::Imbrex(_) => "=",
        }
    }
}
