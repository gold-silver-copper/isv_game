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
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Sky => Color::Rgb(239, 240, 235),
        }
    }
}
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Floor {
    Vȯzduh, //air
    Zemja,  //earth
    Voda,   //water
    Trava,  // grass
    Burjan, //grass type
    Kovylj, //grass type
    Pěsȯk,  //sand
    Mělj,   //sandbank
    Il,     //clay
    Glina,  //clay
    Blåto,  //bog
}

impl Floor {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Vȯzduh => Color::Rgb(239, 240, 235),
            Self::Zemja => Color::Rgb(155, 118, 83),
            Self::Voda => Color::Rgb(15, 94, 156),
            Self::Trava => Color::Rgb(65, 152, 1),
            Self::Burjan => Color::Rgb(19, 109, 21),
            Self::Kovylj => Color::Rgb(30, 115, 40),
            Self::Pěsȯk => Color::Rgb(242, 210, 169),
            Self::Mělj => Color::Rgb(232, 200, 159),
            Self::Il => Color::Rgb(135, 133, 131),
            Self::Glina => Color::Rgb(145, 143, 141),
            Self::Blåto => Color::Rgb(155, 153, 161),
        }
    }

    pub fn to_displaychar(&self) -> String {
        match &self {
            Self::Vȯzduh => " ".into(),
            Self::Zemja => "%".into(),
            Self::Voda => "~".into(),
            Self::Trava => ",".into(),
            Self::Burjan => ";".into(),
            Self::Kovylj => ":".into(),
            Self::Pěsȯk => ".".into(),
            Self::Mělj => ".".into(),
            Self::Il => ".".into(),
            Self::Glina => ".".into(),
            Self::Blåto => ".".into(),
        }
    }

    pub fn to_front_color(&self) -> Color {
        match &self {
            Self::Vȯzduh => Color::Rgb(229, 240, 235),
            Self::Zemja => Color::Rgb(145, 118, 83),
            Self::Voda => Color::Rgb(10, 84, 146),
            Self::Trava => Color::Rgb(65, 142, 1),
            Self::Burjan => Color::Rgb(19, 99, 21),
            Self::Kovylj => Color::Rgb(30, 105, 40),
            Self::Pěsȯk => Color::Rgb(242, 200, 169),
            Self::Mělj => Color::Rgb(232, 190, 159),
            Self::Il => Color::Rgb(135, 123, 131),
            Self::Glina => Color::Rgb(145, 133, 141),
            Self::Blåto => Color::Rgb(155, 143, 161),
        }
    }
}

impl Voxel {
    pub fn to_graphic(&self) -> GraphicTriple {
        let voxel_character: String = self.floor.to_displaychar();
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
