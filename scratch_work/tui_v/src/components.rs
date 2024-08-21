use crate::*;

#[derive(Default, Debug)]
pub struct ComponentHolder {
    pub positions: HashMap<EntityID, MyPoint>,
    pub ent_types: HashMap<EntityID, EntityType>,
    pub inventories: HashMap<EntityID, Inventory>,
}
