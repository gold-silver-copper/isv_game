use crate::*;

pub type CoordinateUnit = i64;
pub const LOCAL_RANGE: i64 = 2000;
pub type MyPoint = (CoordinateUnit, CoordinateUnit);
pub type GraphicTriple = (String, RatColor, RatColor);


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

#[derive(Debug)]
pub struct RenderPacket {
    pub voxel_grid: Vec<Vec<GraphicTriple>>,
}

impl RenderPacket {
    pub fn new() -> Self {
        RenderPacket {
            voxel_grid: Vec::new(),
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
