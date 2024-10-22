use rand::Rng;

use crate::*;

#[derive(Default)]
pub struct ComponentHolder {
    pub positions: HashMap<EntityID, MyPoint>,
    pub ent_types: HashMap<EntityID, EntityType>,
    pub equipments: HashMap<EntityID, Equipment>,
    pub healths: HashMap<EntityID, Health>,

    pub stats: HashMap<EntityID, Stats>,
}

pub type EntSet = HashSet<EntityID>;
#[derive(Default)]
pub struct Equipment {
    pub equipped: EntSet,
    pub inventory: EntSet,
    pub ranged_weapon: RangedWeapon,
    pub arrows: i32,
    pub darts: i32,
    pub bullets: i32,
    pub javelins: i32,
}
#[derive(Default)]
pub struct Stats {
    pub strength: i32,
    pub speed: i32,
    pub intelligence: i32,
}

//try again
pub struct Health {
    pub current_health: i32,
}
pub struct Name {
    pub first_name: String,
    pub last_name: String,
}
impl Default for Name {
    fn default() -> Self {
        Name {
            first_name: "Ivan".into(),
            last_name: "Vragov".into(),
        }
    }
}

impl Default for Health {
    fn default() -> Self {
        Health { current_health: 1 }
    }
}

impl App {
    pub fn get_unique_eid(&mut self) -> EntityID {
        self.entity_counter += 1;
        self.entity_counter.clone()
    }

    pub fn get_ent_type(&self, eid: &EntityID) -> EntityType {
        self.components.ent_types.get(eid).unwrap().clone()
    }

    pub fn get_entity_name(&self, subj: &EntityID) -> String {
        let ent_typ = self.get_ent_type(subj);

        let stringik = match ent_typ {
            EntityType::Human(pro) => format!("{pro}"),
            EntityType::Animal(anim) => {
                format!("{anim}")
            }
            EntityType::Item(itemik) => itemik.item_name(),
        };

        stringik
    }
    pub fn create_item(&mut self, item: ItemType) -> EntityID {
        let eid = self.get_unique_eid();

        self.components
            .ent_types
            .insert(eid.clone(), EntityType::Item(item));

        eid
    }

    pub fn set_ent_position(&mut self, eid: &EntityID, point: &MyPoint) {
        self.components.positions.insert(eid.clone(), point.clone());

        let voxik = self
            .game_map
            .get_mut_voxel_at(point)
            .expect("cant spawn ent in empty voxel");

        voxik.entity_set.push(eid.clone());
    }

    pub fn spawn_item_at(&mut self, point: &MyPoint, item: ItemType) -> EntityID {
        let eid = self.create_item(item);
        self.set_ent_position(&eid, point);

        eid
    }
    pub fn spawn_animal_at(&mut self, animal_type: &AnimalType, point: &MyPoint) -> EntityID {
        let eid = self.get_unique_eid();
        let maxik = animal_type.max_stat();
        let strength = self.small_rng.gen_range(1..=maxik + 1);
        let intik = self.small_rng.gen_range(1..=maxik + 1);
        let speedik = self.small_rng.gen_range(1..=maxik + 1);
        self.components.positions.insert(eid.clone(), point.clone());
        self.components
            .ent_types
            .insert(eid.clone(), EntityType::Animal(animal_type.clone()));

        self.components.healths.insert(
            eid.clone(),
            Health {
                current_health: strength as i32 * 10,
            },
        );

        self.components.stats.insert(
            eid.clone(),
            Stats {
                strength: strength as i32,
                speed: speedik as i32,
                intelligence: intik as i32,
            },
        );

        let voxik = self
            .game_map
            .get_mut_voxel_at(point)
            .expect("cant spawn ent in empty voxel");

        voxik.entity_set.push(eid.clone());

        eid.clone()
    }
    pub fn spawn_human_at(&mut self, point: &MyPoint, profession: Profession) -> EntityID {
        let eid = self.get_unique_eid();
        let maxik = profession.skill_level();
        let strength = self.small_rng.gen_range(1..=maxik + 1);
        let intik = self.small_rng.gen_range(1..=maxik + 1);
        let speedik = self.small_rng.gen_range(1..=maxik + 1);
        let equipik = profession.random_equip();
        let wepik = profession.random_weapon();
        self.components.positions.insert(eid.clone(), point.clone());
        self.components
            .ent_types
            .insert(eid.clone(), EntityType::Human(profession));

        let mut equi = Equipment::default();

        for wep in wepik {
            if self.small_rng.gen_bool(0.3) {
                equi.equipped
                    .insert(self.create_item(ItemType::Weapon(wep)));
            }
        }

        for arm in equipik {
            if self.small_rng.gen_bool(0.3) {
                equi.equipped
                    .insert(self.create_item(ItemType::Clothing(arm)));
            }
        }

        self.components.equipments.insert(eid.clone(), equi);

        self.components.healths.insert(
            eid.clone(),
            Health {
                current_health: strength as i32 * 10,
            },
        );

        self.components.stats.insert(
            eid.clone(),
            Stats {
                strength: strength as i32,
                speed: speedik as i32,
                intelligence: intik as i32,
            },
        );

        let voxik = self
            .game_map
            .get_mut_voxel_at(point)
            .expect("cant spawn ent in empty voxel");

        voxik.entity_set.push(eid.clone());

        eid.clone()
    }
    pub fn spawn_player_at(&mut self, point: &MyPoint) -> EntityID {
        let pid = self.spawn_human_at(point, Profession::Rycaŕ);

        let iid = self.create_item(ItemType::Weapon(Weapon::Meč));
        let iid2 = self.create_item(ItemType::Clothing(Clothing::Toga));
        let iid3 = self.create_item(ItemType::Weapon(Weapon::Bulava));
        let iid4 = self.create_item(ItemType::Consumable(Consumable::Lěkaŕstvo));
        let iid5 = self.create_item(ItemType::Consumable(Consumable::Pivo));
        let iid6 = self.create_item(ItemType::Consumable(Consumable::Hlěb));

        let player_equip = self
            .components
            .equipments
            .get_mut(&pid)
            .expect("MUST HAVE QUEIP");
        player_equip.arrows = 200;
        player_equip.bullets = 20;
        player_equip.javelins = 300;
        player_equip.equipped.insert(iid);
        player_equip.inventory.insert(iid2);
        player_equip.inventory.insert(iid3);
        player_equip.inventory.insert(iid4);
        player_equip.inventory.insert(iid5);
        player_equip.inventory.insert(iid6);

        pid
    }
}
