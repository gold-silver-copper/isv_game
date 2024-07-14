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
                Some(roof) => (roof.symbol(), roof.to_fg_color(), roof.to_bg_color()),
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
pub enum Floor {
    Liquid(LiquidType),
    Earth(EarthType),
}

impl Floor {
    pub fn fg_color(&self) -> RatColor{
        RatColor::Gray
    }
    pub fn bg_color(&self) -> RatColor{
        match self {
            Floor::Liquid(liq) => {liq.color()},
            Floor::Earth(ear) => {ear.color()},

        }
    }
    pub fn symbol(&self) -> String{
        match self {
            Floor::Liquid(_) => "~".into(),
            Floor::Earth(_) => ".".into(),

        }
    }

    pub fn to_graphic_triple(&self) -> GraphicTriple {


        (self.symbol(),self.fg_color(),self.bg_color())


    }

}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum LiquidType {
   Water,
   Lava,
   Beer

}



impl LiquidType {
    pub fn color(&self) -> RatColor{
        match self {
            LiquidType::Water =>  RatColor::Rgb(35, 137, 218),
            LiquidType::Lava =>  RatColor::Rgb(135, 37, 118),
            LiquidType::Beer =>  RatColor::Rgb(35, 37, 118),


        }
    }


}

impl EarthType {
    pub fn color(&self) -> RatColor{
        match self {
            EarthType::Dirt =>  RatColor::Rgb(145, 118, 83),
            EarthType::Clay =>  RatColor::Rgb(145, 118, 83),
            EarthType::Sand =>  RatColor::Rgb(145, 118, 83),


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
   Tegula,
   Imbrex,

}





#[derive(Clone, Debug, PartialEq)]
pub enum SolidMaterial {
    Wood(Tree),
    Stone(Mineral),
    Metal(Metal),
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
            Tree::Glinos => RatColor::Rgb(51,34,17)
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
            _ => RatColor::Magenta
        }
    }
}
impl Metal {

    pub fn color(&self) -> RatColor {

        match self {
            _ => RatColor::Magenta
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
        match &self {
           // RoofType::Wall(sm) => sm.to_color(),
            _ => RatColor::Black
        }
    }
    pub fn to_bg_color(&self) -> RatColor {
        match &self {
           // RoofType::Wall(sm) => sm.to_color(),
            _ => RatColor::White
        }
    }
    pub fn symbol(&self) -> String{
        match self {
            Roof::Tegula => "t".into(),
            Roof::Imbrex => "i".into(),

        }
    }
}

