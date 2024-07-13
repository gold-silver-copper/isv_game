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
            Some(furn) => (furn.symbol.into(), furn.to_color(), floor.2.clone()),
            None => floor,
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
#[derive(Clone, Debug, PartialEq)]
pub struct Floor {
    pub name: &'static str,
    pub symbol: &'static str,
    pub fg_color: RatColor,
    pub bg_color: RatColor,
    pub floor_type: FloorType,
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Furniture {
    pub name: &'static str,
    pub symbol: &'static str,

    pub furniture_type: FurnitureType,
}

impl Furniture {
    pub fn to_color(&self) -> RatColor {
        match &self.furniture_type {
            FurnitureType::Wall(sm) => sm.to_color(),
            FurnitureType::Door(sm) => sm.to_color(),
        }
    }
}

impl Floor {
    pub fn to_graphic_triple(&self) -> GraphicTriple {
        (
            self.symbol.into(),
            self.fg_color.into(),
            self.bg_color.into(),
        )
    }
}
