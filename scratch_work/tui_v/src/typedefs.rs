use crate::*;

pub type CoordinateUnit = i64;
pub type EntityID = i64;
pub const LOCAL_RANGE: i64 = 10000;
pub const MAP_SIZE: i64 = 100;
pub const WALL_FURNITURE: Furniture = Furniture::Wall(SolidMaterial::Wood(Tree::Glinos));
pub const TEGULA_ROOF: Roof = Roof::Tegula(SolidMaterial::Wood(Tree::Glinos));

pub const DIRT_FLOOR: Floor = Floor::Earth(EarthType::Dirt);
pub const SAND_FLOOR: Floor = Floor::Earth(EarthType::Sand);
pub const CLAY_FLOOR: Floor = Floor::Earth(EarthType::Clay);

pub const WATER_FLOOR: Floor = Floor::Liquid(LiquidType::Water);

pub struct Player;

pub type MyPoint = (CoordinateUnit, CoordinateUnit);
pub type GraphicTriple = (&'static str, RatColor, RatColor);



pub struct GraphicComponent(pub GraphicTriple);

// for horse vehicle use horse chess piece as head  ∧ ♞

// projectile weapons have two parts (path to take, and steps of path to take, something flying slow and not far will have low each, something flying slow but far will ahve lots of steps, this can be passed to Go()
//MAKE IT POSSIBLE TO SET PLAYER AI MODE FOR AUTO PLAYING
#[derive(Clone, Debug, PartialEq)]
pub enum GameAction {
    Wait,
    Take(),
    MeleeAttack(),
    Drop(),
    Give(),
    Hit(),
    Go(CardinalDirection),
    Quit,
}



pub fn add_two_points(p1: &MyPoint, p2: &MyPoint) -> MyPoint {
    let mut result = (0, 0);
    result.0 = p1.0 + p2.0;
    result.1 = p1.1 + p2.1;

    result
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

pub type ActionMap = HashMap<EntityID,GameAction>;






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
