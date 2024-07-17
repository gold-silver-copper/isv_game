use crate::*;



#[derive(Clone, Debug, Display, PartialEq)]
pub enum EarthType {
    Dirt,
    Clay,
    Sand,
}

#[derive(Clone, Debug, Display, PartialEq, Component)]
pub enum EntityType {
    Creatura(CreatureType),
   //Item(ItemType),
}

// fishes, amphibians, reptiles, birds, and mammals.
#[derive(Clone, Debug, Display, PartialEq, Component)]
pub enum CreatureType {
    Human,
    Mammal(MammalType),
    Reptile,
    Bird, 
    Fish,
    Amphibian,







}


#[derive(Clone, Debug, Display, PartialEq, Component)]
pub enum MammalType {
    Camelus,//camel
    Er,//hedgehog
    Mus, //Mouse
    Ursus, //Bear
    Zibethus, //Civet
    Elephantus, //Elephant
    Cuniculus, //A type of rabbit
    Dasypus, //Rabbit
    Lepus, // Hare
    Camelopardalis, // Giraffe
    Dorcas, //Gazelle
    Hippopotamus, // hippo
    Musimo, //Wild sheep in sardinia
    Addax, //addax type of antelope
    Gazella, //Gazelle
    Oryx, //oryx
    Pygargus,
    Tragelaphus, //Type of goat
    Bison,
    Bonasus, //European bison
    Bubalus, // buffalo
    Ceva, //Cow
    Bos, //Cow
    Taurus,
    Buccus, //bull
    Haedus,
    Caper, //goat
    Caprea, //goat

    Hircus, //goat
    Ibex, // goat
    Camox,
    Umber,
    Agna,
    Agnus,
    Apica,
    Aries,
    Ovis,
Alces, // Elk
Cervus, //Stag deer
Damma, //Fallow deer
Gammus, //Kind of Deer
Hinnuleus, //Young mule
Rangifer, //reindeer
Tarandus, //reindeer
Aper, //wild boar
Porcus,
Porca,
Sus,
Hyaena,
Belbus,







}

#[derive(Clone, Debug, Display, PartialEq, Component)]
pub enum BirdType {
    Vespertilio, //flying bat





}


#[derive(Clone, Debug, Display, PartialEq, Component)]
pub enum FishType {
    Ballaena, //whale
    Physeter, //sperm whale
    Delphinus,
    Orca,
    Pistris,
    Tursio,











}





#[derive(Clone, Debug, Display, PartialEq, Component)]
pub enum ItemType {
    Clothing, 
    Weapon,
    Consumable,
    Armor,



}

impl EntityType {

    pub fn to_complex_noun(&self) -> ComplexNoun {

        match self {

            _ => { let head_noun = format!("{}",self).to_lowercase();
            //println!("{}",head_noun);
        ComplexNoun{head_noun, ..default()}}
        }

    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Roof {
    Tegula(SolidMaterial),
    Imbrex(SolidMaterial),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SolidMaterial {
    Wood(Tree),
    Stone(Mineral),
    Metal(Metal),
    Terracotta(EarthType),
}



#[derive(Clone, Debug, PartialEq)]
pub enum Tree {
    Glinos, //Maple Tree
}



#[derive(Clone, Debug, PartialEq)]
pub enum Mineral {
    Iaspis, //Gold
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metal {
    Glinos, //Maple Tree
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Furniture {
    Wall(SolidMaterial),
    Door(SolidMaterial),
    Trinket,
}

