use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Voxel {
    pub floor: Floor,
    pub roof: Option<Roof>,
    pub furniture: Option<Furniture>,
    pub voxel_pos: MyPoint,
}

impl Voxel {
    pub fn to_graphic(&self, with_roof: bool) -> GraphicTriple {
        let mut floor = self.floor.to_graphic_triple();
        let mut plus_furn: GraphicTriple = match &self.furniture {
            Some(furn) => (furn.symbol.into(), furn.to_color(), floor.2.clone()),
            None => floor,
        };

        if with_roof {
            let mut plus_roof: GraphicTriple = match &self.roof {
                Some(roof) => (roof.symbol.into(), roof.to_fg_color(), roof.to_bg_color()),
                None => plus_furn,
            };
    
            plus_roof
        }
        else {plus_furn}

     
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
pub enum FloorType {
    Liquid(LiquidType),
    Earth(EarthType),
}

impl FloorType {
    pub fn fg_color(&self) -> RatColor{
        match self {
            FloorType::Liquid(liq) => {liq.fg_color()},
            FloorType::Earth(ear) => {ear.fg_color()},

        }
    }
    pub fn bg_color(&self) -> RatColor{
        match self {
            FloorType::Liquid(liq) => {liq.bg_color()},
            FloorType::Earth(ear) => {ear.bg_color()},

        }
    }

}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum LiquidType {
   Water,
   Lava,
   Beer

}



impl LiquidType {
    pub fn fg_color(&self) -> RatColor{
        match self {
            LiquidType::Water =>  RatColor::Rgb(35, 137, 218),
            LiquidType::Lava =>  RatColor::Rgb(135, 37, 118),
            LiquidType::Beer =>  RatColor::Rgb(35, 37, 118),


        }
    }
    pub fn bg_color(&self) -> RatColor{
        match self {
            LiquidType::Water =>  RatColor::Rgb(35, 137, 218),
            LiquidType::Lava =>  RatColor::Rgb(135, 37, 118),
            LiquidType::Beer =>  RatColor::Rgb(35, 37, 118),

        }
    }

}

impl EarthType {
    pub fn fg_color(&self) -> RatColor{
        match self {
            EarthType::Dirt =>  RatColor::Rgb(145, 118, 83),
            EarthType::Clay =>  RatColor::Rgb(145, 118, 83),
            EarthType::Sand =>  RatColor::Rgb(145, 118, 83),


        }
    }
    pub fn bg_color(&self) -> RatColor{
        match self {
            EarthType::Dirt =>  RatColor::Rgb(145, 118, 83),
            EarthType::Clay =>  RatColor::Rgb(145, 118, 83),
            EarthType::Sand => RatColor::Rgb(145, 118, 83),

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
pub enum RoofType {
   Tegula,
   Imbrex,

}
#[derive(Clone, Debug, PartialEq)]
pub struct Floor {
    pub name: &'static str,
    pub symbol: &'static str,
    pub floor_type: FloorType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Roof {
    pub name: &'static str,
    pub symbol: &'static str,
    pub roof_type: RoofType,
}

impl Default for Floor {
    fn default() -> Self {

        Self {
            name: "pavimentum",
            symbol: " ",
         
            floor_type: FloorType::Earth(EarthType::Dirt)
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum SolidMaterial {
    Wood(Tree),
    Stone(Mineral),
    Metal(Alloy),
}

impl SolidMaterial {
    pub fn to_color(&self) -> RatColor {
        match self {
            Self::Wood(inner) => inner.fg_color.clone(),
            Self::Stone(inner) => inner.fg_color.clone(),
            Self::Metal(inner) => inner.fg_color.clone(),
            _ => todo!("IMPLEMENT COLORS FOR MATERIAL"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tree {
    pub name: &'static str,
    pub symbol: &'static str,
    pub fg_color: RatColor,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mineral {
    pub name: &'static str,
    pub symbol: &'static str,
    pub fg_color: RatColor,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Alloy {
    pub name: &'static str,
    pub symbol: &'static str,
    pub fg_color: RatColor,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum FurnitureType {
    Wall(SolidMaterial),
    Door(SolidMaterial),
    Trinket,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Furniture {
    pub name: &'static str,
    pub symbol: &'static str,

    pub furniture_type: FurnitureType,
}

impl Default for Furniture {
    fn default() -> Self {

        Self {
            name: "supellex",
    symbol: "|",
    furniture_type: FurnitureType::Trinket,
        }
    }
}

impl Furniture {
    pub fn to_color(&self) -> RatColor {
        match &self.furniture_type {
            FurnitureType::Wall(sm) => sm.to_color(),
            FurnitureType::Door(sm) => sm.to_color(),
            _ => RatColor::White
        }
    }
}

impl Roof {
    pub fn to_fg_color(&self) -> RatColor {
        match &self.roof_type {
           // RoofType::Wall(sm) => sm.to_color(),
            _ => RatColor::Black
        }
    }
    pub fn to_bg_color(&self) -> RatColor {
        match &self.roof_type {
           // RoofType::Wall(sm) => sm.to_color(),
            _ => RatColor::White
        }
    }
}

impl Floor {
    pub fn to_graphic_triple(&self) -> GraphicTriple {
        (
            self.symbol.into(),
            self.floor_type.fg_color(),
            self.floor_type.bg_color(),
        )
    }
}
