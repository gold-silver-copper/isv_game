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

#[derive(Clone, Debug, PartialEq)]
pub struct EquipmentComponent {
    pub melee_weapon: Option<MeleeWeapon>,

    pub ranged_weapon: Option<RangedWeapon>,

    pub head: Option<ClothingItem>,
    pub shoulders: Option<ClothingItem>,
    pub torso: Option<ClothingItem>,
    pub legs: Option<ClothingItem>,
}

impl EquipmentComponent {
    pub fn new_empty() -> EquipmentComponent {
        EquipmentComponent {
            melee_weapon: None,

            ranged_weapon: None,

            head: None,
            shoulders: None,
            torso: None,
            legs: None,
        }
    }
    pub fn new_hunter() -> EquipmentComponent {
        EquipmentComponent {
            melee_weapon: Some(MeleeWeapon {
                weapon_type: MeleeWeaponType::Nož,
                material_type: SolidMaterial::Metal(MetalType::Bronza),
            }),

            ranged_weapon: Some(RangedWeapon {
                weapon_type: RangedWeaponType::Lųk,
                tetiva_material: FabricMaterial::Koža(MammalType::Jelenj),
                rema_material: WoodType::Jasenj,
                ammo_material: SolidMaterial::Metal(MetalType::Bronza),
            }),

            head: Some(ClothingItem {
                clothing_type: ClothingType::Head(HeadClothingType::Kapuc),
                fabric_type: FabricMaterial::Tkanina(PlantType::Trava(GrassType::Jasenėc)),
            }),
            shoulders: None,
            torso: None,
            legs: None,
        }
    }
}

pub type InventoryComponent = Vec<ItemType>;
