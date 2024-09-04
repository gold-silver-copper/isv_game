use crate::*;

pub fn first_char(s: &str) -> &str {
    &s[..s.char_indices().skip(1).next().map_or(s.len(), |(i, _)| i)]
}

pub type CoordinateUnit = i64;
pub type EntityID = i64;
pub type Subject = EntityID;
pub type DirectObject = EntityID;
pub type IndirectObject = EntityID;
pub type InstrumentObject = EntityID;

pub const LOCAL_RANGE: i64 = 10000;
pub const MAP_SIZE: i64 = 100;

pub const CURSOR_UP: char = 'w';
pub const CURSOR_UP_LEFT: char = 'q';
pub const CURSOR_UP_RIGHT: char = 'e';
pub const CURSOR_DOWN: char = 's';
pub const CURSOR_DOWN_LEFT: char = 'z';
pub const CURSOR_DOWN_RIGHT: char = 'c';
pub const CURSOR_LEFT: char = 'a';
pub const CURSOR_RIGHT: char = 'd';
pub const INVENTORY_MENU: char = 'i';
pub const WAIT_KEY: char = 't';
pub const DROP_UNEQUIP_ACTION: char = 'h';
pub const PICKUP_EQUIP_ACTION: char = 'j';
pub const QUIT_BACK: char = ']';

//pub const WALL_FURNITURE: Furniture = Furniture::Wall(Tree::Glinos);

pub const BASIC_WOOD_MATERIAL: Material = Material::Wood(Tree::Glinos);
pub const BASIC_METAL_MATERIAL: Material = Material::Metal(Metal::Gold);

pub const WOOD_WALL: Furniture = Furniture::Wall(BASIC_WOOD_MATERIAL);
pub const TEGULA_ROOF: Roof = Roof::Tegula(BASIC_METAL_MATERIAL);
pub const GRAVEL_FLOOR: Floor = Floor::Gravel(BASIC_METAL_MATERIAL);

pub type MyPoint = (CoordinateUnit, CoordinateUnit);
pub type GraphicTriple = (String, RatColor, RatColor);

// Define a marker trait

#[derive(Clone, Debug, PartialEq)]
pub enum GameAction {
    Wait(Subject),
    PickUp(Subject, DirectObject),
    Equip(Subject, DirectObject),
    UnEquip(Subject, DirectObject),
    BumpAttack(Subject, DirectObject),
    MeleeAttack(),
    Drop(Subject, DirectObject),
    Give(),

    Go(Subject, CardinalDirection),
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

pub type ActionVec = Vec<GameAction>;

pub type RenderPacket = Vec<Vec<GraphicTriple>>;

#[derive(Clone, Debug, PartialEq)]
pub enum CardinalDirection {
    North,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    West,
    South,
    East,
}

impl CardinalDirection {
    pub fn to_xyz(&self) -> MyPoint {
        match self {
            CardinalDirection::North => (0, 1),
            CardinalDirection::NorthEast => (1, 1),
            CardinalDirection::NorthWest => (-1, 1),
            CardinalDirection::SouthEast => (1, -1),
            CardinalDirection::SouthWest => (-1, -1),
            CardinalDirection::West => (-1, 0),
            CardinalDirection::South => (0, -1),
            CardinalDirection::East => (1, 0),
        }
    }
    pub fn to_isv(&self) -> &'static str {
        match self {
            CardinalDirection::North => "sěver",
            CardinalDirection::West => "zapad",
            CardinalDirection::South => "jug",
            CardinalDirection::East => "vȯzhod",
            CardinalDirection::NorthEast => "sěverovȯzhod",
            CardinalDirection::NorthWest => "sěverozapad",
            CardinalDirection::SouthEast => "jugovȯzhod",
            CardinalDirection::SouthWest => "jugozapad",
        }
    }
}
