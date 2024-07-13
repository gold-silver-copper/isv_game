use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Voxel {
    pub floor: Floor,
pub furniture: Option<Furniture>,
    pub voxel_pos: MyPoint,
}





impl Voxel {
    pub fn to_graphic(&self) -> GraphicTriple {
    
    let mut floor = self.floor.to_graphic_triple();
    let mut plus_furn: GraphicTriple = match &self.furniture {
        Some(furn) => (furn.symbol.into(),furn.fg_color.clone(),floor.2.clone()),
        None => floor
    };

    plus_furn
    
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
    Water,
    Dirt,
 
}
#[derive(Clone, Debug,  PartialEq)]
pub struct Floor {
    pub name: &'static str,
    pub symbol: &'static str,
    pub fg_color: RatColor,
    pub bg_color: RatColor,
    pub floor_type: FloorType
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum FurnitureType {
    Wall,
    Door,

 
}

#[derive(Clone, Debug,  PartialEq)]
pub struct Furniture {
    pub name: &'static str,
    pub symbol: &'static str,
    pub fg_color: RatColor,
    
    pub furniture_type: FurnitureType
}

pub const WALL_FURNITURE:Furniture = Furniture{name: "paries",symbol: "#", fg_color:RatColor::Rgb(5, 3, 2),furniture_type:FurnitureType::Wall};

/*
dirt,brěst,solid, ,"166,112,78","166,112,78"
water,jasenj,liquid,~,"","4"
sand,lipa,granular, ,"233,225,194","233,225,194"
grass,jablånj,solid, ,"21,114,65","21,114,65"
*/
pub const DIRT_FLOOR: Floor = Floor{name: "lutum", symbol: "%",fg_color: RatColor::Rgb(145, 118, 83),bg_color: RatColor::Rgb(155, 118, 83), floor_type:FloorType::Dirt};
pub const WATER_FLOOR: Floor = Floor{name: "aqua", symbol: "~",fg_color: RatColor::Rgb(35,137,218),bg_color: RatColor::Rgb(45,117,228), floor_type:FloorType::Water};



impl Floor {
    pub fn to_graphic_triple(&self) -> GraphicTriple{

        (self.symbol.into(),self.fg_color.into(),self.bg_color.into())
    }
}