use crate::*;

#[derive(Default)]
pub struct ComponentHolder {
    pub positions: HashMap<EntityID, MyPoint>,
    pub ent_types: HashMap<EntityID, EntityType>,
    pub equipments: HashMap<EntityID, Equipment>,
    pub containers: HashMap<EntityID, Container>,
}

pub type Container = HashSet<EntityID>;

#[derive(Default)]
pub struct Equipment {
    pub wielding: HashSet<EntityID>,
    pub wearing: HashSet<EntityID>,
}
