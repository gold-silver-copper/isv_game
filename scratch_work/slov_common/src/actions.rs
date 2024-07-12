use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Wait,
    Take(AccusativeID),
    MeleeAttack(AccusativeID),
    Drop(ItemType),
    Give(AccusativeID, DativeID),
    Hit(AccusativeID, InstrumentalID),
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

    pub fn melee_attack(world: &mut MyWorld, subject: &EntityID, object: &EntityID) -> SuccessType {
        //check if ents are within distance
        //get subj and obj locations
        let sub_loc = world.ent_loc_index.get(subject).unwrap_or(&(0, 0));
        let obj_loc = world.ent_loc_index.get(object).unwrap_or(&(420, 420));

        let disbet = sub_loc.distance_2(obj_loc);
        let melee_range = match world.entity_map.get(subject).unwrap_or(&EntityType::None) {
            EntityType::Human(x) => {
                if let Some(boop) = &x.equipment.melee_weapon {
                    boop.weapon_type.weapon_range()
                } else {
                    1
                }
            }
            EntityType::Monster(_) => 3,
            _ => 0,
        };

        if disbet <= melee_range {
            return SuccessType::Success;
        } else {
            return SuccessType::Failure;
        }

        //do damage to object
        //implement system for checking if health is below 0
    }

    pub fn take(world: &mut MyWorld, subject: &EntityID, object: &EntityID) -> SuccessType {
        //get subj and obj locations
        let sub_loc = world.ent_loc_index.get(subject);
        let obj_loc = world.ent_loc_index.get(object);

        //if the subj and obj are in the same place
        if sub_loc == obj_loc {
            let mut nun = EntityType::None;
            let mut nun2 = EntityType::None;

            let mut itik = world.entity_map.get(object).unwrap_or(&mut nun2).clone();
            let mut boop = world.entity_map.get_mut(subject).unwrap_or(&mut nun);
            //if the object is an item
            match itik {
                EntityType::Item(itimik) => {
                    match boop {
                        //if the subject is a player
                        EntityType::Human(pla) => {
                            pla.inventory.push(itimik.clone());
                            world.delete_entity(object);
                            return SuccessType::Success;
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        SuccessType::Failure
    }
    pub fn drop(world: &mut MyWorld, subject: &EntityID, item_to_drop: &ItemType) -> SuccessType {
        let sub_loc = world.ent_loc_index.get(subject).unwrap_or(&(0, 0)).clone();

        //does ent contain item

        let mut nun = EntityType::None;

        let mut meow = SuccessType::Failure;

        let ent_to_check = world.entity_map.get_mut(subject).unwrap_or(&mut nun);

        match ent_to_check {
            EntityType::Human(igrok) => {
                meow = remove_first_instance(&mut igrok.inventory, &item_to_drop.clone());
            }
            _ => (),
        }

        if meow == SuccessType::Success {
            world.new_entity(&sub_loc, &EntityType::Item(item_to_drop.clone()));
        }

        meow
    }
}
