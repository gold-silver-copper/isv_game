use crate::*;

#[derive(Default)]
pub struct ComponentHolder {
    pub positions: HashMap<EntityID, MyPoint>,
    pub ent_types: HashMap<EntityID, EntityType>,
    pub equipments: HashMap<EntityID, Equipment>,
    pub healths: HashMap<EntityID, Health>,
}

pub type EntSet = HashSet<EntityID>;
#[derive(Default)]
pub struct Equipment {
    pub equipped: EntSet,
    pub inventory: EntSet,
}

pub struct Health {
    current_health: i64,
    max_health: i64,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            current_health: 80,
            max_health: 100,
        }
    }
}
