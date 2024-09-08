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

    Ammo(Ammo),
    Clothing(Clothing),
    Accessory(Accessory),
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
    Sword,
    Mace,
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
    Kulja(i64),

    Strěla(i64),
}
#[derive(Clone, Debug, PartialEq, Display)]
pub enum Clothing {
    Toga,
    Helma,
}
#[derive(Clone, Debug, PartialEq, Display)]
pub enum Accessory {
    Kolce,
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
