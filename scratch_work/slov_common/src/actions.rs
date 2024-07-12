use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Wait,

    Go(LocativeID),
    Quit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SuccessType {
    Success,
    Failure,
}
#[derive(Clone, Debug)]
pub struct ActionPacket {
    pub action: ActionType,
    pub success: SuccessType,
    pub action_location: MyPoint,
    pub action_subject: EntityID,
}

impl RTreeObject for ActionPacket {
    type Envelope = AABB<(i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.action_location.0, self.action_location.1))
    }
}

impl PointDistance for ActionPacket {
    fn distance_2(&self, point: &(i64, i64)) -> i64 {
        self.action_location.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64)) -> bool {
        self.action_location.contains_point(point)
    }
}

#[derive(Clone, Debug)]
pub struct Action {}

impl Action {
    pub fn go(world: &mut MyWorld, subject: &EntityID, destination: &LocativeID) -> SuccessType {
        match destination {
            LocativeID::Cardinal(cd) => world.move_entity_in_direction(subject, cd), //world.move_entity_in_direction(subject, cd),
            LocativeID::Entity(_) => {
                panic!("not implemented ent")
            }
            LocativeID::Point(_) => {
                panic!("not implemented point")
            }
        }
    }
}
