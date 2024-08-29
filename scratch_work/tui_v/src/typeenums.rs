use crate::*;

#[derive(Display,PartialEq)]
pub enum EntityType {
    Human,
    Item(ItemType),
}
pub enum InputState {
    Basic,
    Inventory,
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
#[derive(Display,PartialEq)]
pub enum ItemType {
    Container(Container),
    Weapon(Weapon),
    Clothing(Clothing),
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
#[derive(Display,Clone)]
pub enum ActionResult {
    Success(GameAction),
    Failure(GameAction),
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
#[derive(Clone, Debug, PartialEq, Display)]
pub enum Clothing {
    Toga,
}
#[derive(Clone, Debug, PartialEq, Display)]
pub enum Container {
    Bag,
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