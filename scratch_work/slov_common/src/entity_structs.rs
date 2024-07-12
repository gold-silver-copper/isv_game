use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Animal {
    pub animal_type: AnimalType,
    pub cur_health: HealthComponent,
    pub max_health: HealthComponent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Human {
    pub inventory: InventoryComponent,
    pub equipment: EquipmentComponent,
    pub cur_health: HealthComponent,
    pub max_health: HealthComponent,
    pub name: NameComponent,
    pub stats: StatsComponent,
}