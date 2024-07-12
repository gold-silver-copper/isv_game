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

#[derive(Clone, Debug, PartialEq)]
pub struct StatsComponent {
    pub sila: StatsUnit,     //sila
    pub bystrost: StatsUnit, // bystrost
    pub razum: StatsUnit,    //razum
}

#[derive(Clone, Debug, PartialEq)]
pub struct NameComponent {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HealthComponent {
    pub health: StatsUnit,      //zdravje
    pub stamina_air: StatsUnit, //vozduh
}

impl HealthComponent {
    pub fn new() -> HealthComponent {
        HealthComponent {
            health: 120,
            stamina_air: 200,
        }
    }
}

impl NameComponent {
    pub fn new() -> NameComponent {
        NameComponent {
            name: String::from("Člověk"),
        }
    }
}

impl StatsComponent {
    pub fn new_default() -> StatsComponent {
        StatsComponent {
            sila: 100,
            bystrost: 100,
            razum: 100,
        }
    }
}


