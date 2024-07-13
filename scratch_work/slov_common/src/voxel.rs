use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Voxel {
    pub roof: Roof,
    pub floor: Floor,

    pub voxel_pos: MyPoint,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Roof {
    Sky,
}
impl Roof {
    pub fn to_color(&self) -> RatColor {
        match &self {
            Self::Sky => RatColor::Rgb(239, 240, 235),
        }
    }
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Floor {
    Water,
    Dirt,
    Sand,
    Grass,
}

impl Floor {
    pub fn to_color(&self) -> RatColor {
        match &self {
            Self::Dirt => RatColor::Rgb(155, 118, 83),
            Self::Water => RatColor::Rgb(15, 94, 156),

            Self::Grass => RatColor::Rgb(19, 109, 21),

            Self::Sand => RatColor::Rgb(242, 210, 169),
        }
    }

    pub fn to_displaychar(&self) -> String {
        match &self {
            Self::Dirt => "%".into(),
            Self::Water => "~".into(),

            Self::Grass => ";".into(),

            Self::Sand => ".".into(),
        }
    }

    pub fn to_front_color(&self) -> RatColor {
        match &self {
            Self::Dirt => RatColor::Rgb(145, 118, 83),
            Self::Water => RatColor::Rgb(10, 84, 146),

            Self::Grass => RatColor::Rgb(19, 99, 21),

            Self::Sand => RatColor::Rgb(242, 200, 169),
        }
    }
}

impl Voxel {
    pub fn to_graphic(&self) -> GraphicTriple {
        let voxel_character= self.floor.to_displaychar();
        let char_color = self.floor.to_front_color();

        let floor_color = self.floor.to_color();

        (voxel_character, char_color, floor_color)
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
