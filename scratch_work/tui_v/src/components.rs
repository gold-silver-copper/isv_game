use crate::*;

#[derive(Default)]
pub struct ComponentHolder {
    pub positions: HashMap<EntityID, MyPoint>,
    pub ent_types: HashMap<EntityID, EntityType>,
    pub equipments: HashMap<EntityID, Equipment>,
}

pub type EntSet = HashSet<EntityID>;
#[derive(Default)]
pub struct Equipment {
    pub equipped: EntSet,
    pub inventory: EntSet,
}
