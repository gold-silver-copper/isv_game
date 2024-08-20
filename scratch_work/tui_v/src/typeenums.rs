use crate::*;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum EarthType {
    Dirt,
   
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum EntityType {
    Animalia,
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
#[derive(Clone, Debug, Display, PartialEq)]
pub enum Roof{
    Tegula(Tree),
}




#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    Glinos, //Maple Tree
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metal {
   Gold
}




#[derive(Clone, Debug, Display, PartialEq)]
pub enum Furniture {
    Wall(Tree),
   
   
}
