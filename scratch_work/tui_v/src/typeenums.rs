use crate::*;

#[derive(Display)]
pub enum EntityType {
    Animalia,
    Item(ItemType),
}

pub enum ItemType {

    Container(Container),
    Weapon(Weapon)

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
// tegula imbrex
#[derive(Display)]
pub enum Roof {
    Tegula(Box<dyn ColoredMaterial>),
}

pub enum Floor {
    Gravel(Box<dyn ColoredMaterial>),
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
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Container {
    Bag,
}

pub enum Furniture {
    Wall(Box<dyn ColoredMaterial>),
}
