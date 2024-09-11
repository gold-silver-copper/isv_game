use crate::*;

#[derive(Display, PartialEq, Clone)]
pub enum EntityType {
    Human,
    Item(ItemType),
}

impl EntityType {
    pub fn is_attackable(&self) -> bool {
        match self {
            EntityType::Human => true,
            EntityType::Item(_) => false,
        }
    }
}
#[derive(Display, PartialEq, Clone)]
pub enum InputState {
    Basic,
    Inventory,
    RangedAttack,
}

pub enum ItemVecType {
    Inventory,
    Ground,
    Equipped,
}

impl Default for ItemVecType {
    fn default() -> Self {
        ItemVecType::Ground
    }
}

impl Default for InputState {
    fn default() -> Self {
        InputState::Basic
    }
}
#[derive(Display, PartialEq, Clone)]
pub enum ItemType {
    Weapon(Weapon),
    Consumable(Consumable),

    Ammo(Ammo),
    Clothing(Clothing),
}
#[derive(Display, PartialEq, Clone)]
pub enum Consumable {
    Lěkaŕstvo,

    Pivo,
}

impl Consumable {
    pub fn health_effect(&self) -> i32 {
        match self {
            Consumable::Lěkaŕstvo => 20,
            Consumable::Pivo => 5,
        }
    }
    pub fn strength_effect(&self) -> i32 {
        match self {
            Consumable::Lěkaŕstvo => 0,
            Consumable::Pivo => 1,
        }
    }
    pub fn int_effect(&self) -> i32 {
        match self {
            Consumable::Lěkaŕstvo => 0,
            Consumable::Pivo => 0,
        }
    }
    pub fn speed_effect(&self) -> i32 {
        match self {
            Consumable::Lěkaŕstvo => 0,
            Consumable::Pivo => 0,
        }
    }
}

impl EntityType {
    pub fn to_complex_noun(&self) -> ComplexNoun {
        match self {
            _ => {
                let head_noun = format!("{}", self).to_lowercase();
                //println!("{}",head_noun);
                ComplexNoun {
                    head_noun,
                    ..ComplexNoun::default()
                }
            }
        }
    }
}

pub enum Material {
    Metal(Metal),
    Wood(Tree),
}

// tegula imbrex
#[derive(Display)]
pub enum Roof {
    Tegula(Material),
}

pub enum Floor {
    Gravel(Material),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    Glinos, //Maple Tree
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metal {
    Gold,
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Weapon {
    Meč,
    Bulava,
}
impl Default for RangedWeapon {
    fn default() -> Self {
        RangedWeapon::Lųk
    }
}
#[derive(Clone, Debug, PartialEq, Display)]
pub enum RangedWeapon {
    Lųk,
    Pråšča,
    Oščěp,
    Drotik,
}
#[derive(Clone, Debug, PartialEq, Display)]
pub enum Ammo {
    Kulja(i32),
    Oščěp(i32),
    Strěla(i32),
    Drotik(i32),
}
#[derive(Clone, Debug, PartialEq, Display)]
pub enum Clothing {
    Toga,
    Helma,
}

pub enum Furniture {
    Wall(Material),
}

#[derive(Clone, Debug, Display, PartialEq, EnumIter, EnumCount, FromRepr)]
pub enum FishType {
    Losos,
    Tunec,
    Karas,
}

#[derive(Clone, Debug, Display, PartialEq, EnumIter, EnumCount, FromRepr)]
pub enum BirdType {
    Sova,
    Vrabec,
    Vran,
    Gavran,
    Kos,
    Gųsę,
}

#[derive(Clone, Debug, Display, PartialEq, EnumIter, EnumCount, FromRepr)]
pub enum LizardType {
    Gad,
    Jaščer,
    Iguana,
    Vųž,
    Žȯlv,
}

#[derive(Clone, Debug, Display, PartialEq, EnumIter, EnumCount, FromRepr)]
pub enum MammalType {
    Los,
    Jelenj,
    Krava,
    Pes,
    Tigr,
}
