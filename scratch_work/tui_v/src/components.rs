use crate::*;

#[derive(Default)]
pub struct ComponentHolder {
    pub positions: HashMap<EntityID, MyPoint>,
    pub ent_types: HashMap<EntityID, EntityType>,
    pub equipments: HashMap<EntityID, EntSet>,
    pub containers: HashMap<EntityID, EntSet>,
}

pub type EntSet = HashSet<EntityID>;


