use crate::*;

#[derive(Default)]
pub struct ComponentHolder {
    pub positions: HashMap<EntityID, MyPoint>,
    pub ent_types: HashMap<EntityID, EntityType>,
    pub equipments: HashMap<EntityID, Equipment>,
    pub healths: HashMap<EntityID, Health>,
    pub genders: HashMap<EntityID, Gender>,
}

pub type EntSet = HashSet<EntityID>;
#[derive(Default)]
pub struct Equipment {
    pub equipped: EntSet,
    pub inventory: EntSet,
}

pub struct Health {
    pub current_health: i64,
    pub max_health: i64,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            current_health: 3,
            max_health: 100,
        }
    }
}
