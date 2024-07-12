use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PositionComponent {
    pub entity_id: EntityID,
    pub point: MyPoint,
}

impl RTreeObject for PositionComponent {
    type Envelope = AABB<(i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.point.0, self.point.1))
    }
}

impl PointDistance for PositionComponent {
    fn distance_2(&self, point: &(i64, i64)) -> i64 {
        self.point.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64)) -> bool {
        self.point.contains_point(point)
    }
}



#[derive(Component)]
pub struct Player;

type XPosition = i64;
type YPosition = i64;
//type ZPosition = i64;

#[derive(Component)]
pub struct GamePosition {
    pub x: XPosition,
    pub y: YPosition,
  //  z: ZPosition
}

impl GamePosition {
    pub fn new() -> GamePosition {

        GamePosition {
            x: 5,
            y:5,
        }
    }
}


#[derive(Component)]
pub struct GameRenderable {
    pub display_char: String,
    pub fg_color: RatColor,
    pub bg_color: RatColor
}

impl GameRenderable {


    pub fn new_human() -> GameRenderable {

        GameRenderable {
            display_char: "@".into(),
            fg_color: RatColor::White,
            bg_color: RatColor::Black,

        }
    }

}