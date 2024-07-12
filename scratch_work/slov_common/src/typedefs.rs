use crate::*;

pub type EntityID = u64;
pub type ItemKey = u16;
pub type AccountID = u64;
pub type StatsUnit = i64;
pub type CoordinateUnit = i64;
pub const LOCAL_RANGE: i64 = 2000;
pub type MyPoint = (CoordinateUnit, CoordinateUnit);
pub type GraphicTriple = (String, Color, Color);

pub type NominativeID = EntityID;
pub type AccusativeID = EntityID;
pub type DativeID = EntityID;
pub type InstrumentalID = EntityID;
pub type PlayerMessage = String;

#[derive(Clone, Debug)]
pub struct GameDataPacket {
    pub entity_info: Vec<EntityPacket>,
    pub voxel_diffs: Vec<Voxel>,
    pub action_info: Vec<ActionPacket>,
}
#[derive(Clone, Debug)]
pub struct EntityPacket {
    pub entity_pos: MyPoint,
    pub entity_type: EntityType,
    pub entity_id: EntityID,
}

pub fn add_two_points(p1: &MyPoint, p2: &MyPoint) -> MyPoint {
    let mut result = (0, 0);
    result.0 = p1.0 + p2.0;
    result.1 = p1.1 + p2.1;

    result
}

///NOTICE THIS ARRAY IS INDEXED Y FIRST FOR PERFORMANCE
pub fn create_2d_array(render_width: usize, render_height: usize) -> Vec<Vec<GraphicTriple>> {
    let grid = vec![vec![(" ".into(), Color::White, Color::Black); render_width]; render_height];
    grid
}

pub fn locate_square(e_pos: &MyPoint, w_radius: i64, h_radius: i64) -> AABB<MyPoint> {
    AABB::from_corners(
        (e_pos.0 - w_radius, e_pos.1 - h_radius),
        (e_pos.0 + w_radius, e_pos.1 + h_radius),
    )
}

#[derive(Debug)]
pub struct RenderPacket {
    pub spans_to_render: Vec<Vec<GraphicTriple>>,

    pub messages_to_render: Vec<PlayerMessage>,
}

impl RenderPacket {
    pub fn new() -> Self {
        RenderPacket {
            spans_to_render: Vec::new(),

            messages_to_render: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LocativeID {
    Cardinal(CardinalDirection),
    Entity(EntityID),
    Point(MyPoint),
}
#[derive(Clone, Debug, PartialEq)]
pub enum CardinalDirection {
    North,
    West,
    South,
    East,
}

impl CardinalDirection {
    pub fn to_xyz(&self) -> MyPoint {
        match self {
            CardinalDirection::North => (0, 1),
            CardinalDirection::West => (-1, 0),
            CardinalDirection::South => (0, -1),
            CardinalDirection::East => (1, 0),
        }
    }
}

pub fn remove_first_instance<T: PartialEq>(vec: &mut Vec<T>, item: &T) -> SuccessType {
    if let Some(index) = vec.iter().position(|x| x == item) {
        vec.remove(index);
        SuccessType::Success
    } else {
        SuccessType::Failure
    }
}
