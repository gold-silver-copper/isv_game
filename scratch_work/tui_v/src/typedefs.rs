use crate::*;

pub type CoordinateUnit = i64;
pub type EntityID = i64;

pub const LOCAL_RANGE: i64 = 10000;
pub const MAP_SIZE: i64 = 100;
//pub const WALL_FURNITURE: Furniture = Furniture::Wall(Tree::Glinos);

pub fn wood_wall() -> Option<Furniture> {
    Some(Furniture::Wall(Box::new(Tree::Glinos)))
}

pub fn tegula_roof() -> Option<Roof> {
    Some(Roof::Tegula(Box::new(Metal::Gold)))
}

pub fn gravel_floor() -> Floor {
    Floor::Gravel(Box::new(Tree::Glinos))
}

pub type MyPoint = (CoordinateUnit, CoordinateUnit);
pub type GraphicTriple = (&'static str, RatColor, RatColor);

// Define a marker trait
pub trait ColoredMaterial: ToColor + Material {}

impl<T: ToColor + Material> ColoredMaterial for T {}

#[derive(Clone, Debug, PartialEq)]
pub enum GameAction {
    Wait,
    PickUp(EntityID),
    MeleeAttack(),
    Drop(EntityID),
    Give(),
    Hit(),
    Go(CardinalDirection),
    Quit,
}

pub fn add_two_points(p1: &MyPoint, p2: &MyPoint) -> MyPoint {
    (p1.0 + p2.0, p1.1 + p2.1)
}

///NOTICE THIS ARRAY IS INDEXED Y FIRST FOR PERFORMANCE
pub fn create_2d_array(render_width: usize, render_height: usize) -> Vec<Vec<GraphicTriple>> {
    let grid =
        vec![vec![(" ".into(), RatColor::White, RatColor::Black); render_width]; render_height];
    grid
}

pub fn locate_square(e_pos: &MyPoint, w_radius: i64, h_radius: i64) -> AABB<MyPoint> {
    AABB::from_corners(
        (e_pos.0 - w_radius, e_pos.1 - h_radius),
        (e_pos.0 + w_radius, e_pos.1 + h_radius),
    )
}

pub type ActionMap = HashMap<EntityID, GameAction>;

#[derive(Debug)]
pub struct RenderPacket {
    pub voxel_grid: Vec<Vec<GraphicTriple>>,
    pub ent_vec: Vec<EntityID>,
}
impl RenderPacket {
    pub fn new() -> Self {
        RenderPacket {
            voxel_grid: Vec::new(),
            ent_vec: Vec::new(),
        }
    }
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
