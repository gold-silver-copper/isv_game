pub enum DrinkType {
    Pivo((Box<dyn Alcoholizable>)),
    Vino((Box<dyn Alcoholizable>)),
    Vodka((Box<dyn Alcoholizable>)),
    Samogon((Box<dyn Alcoholizable>)),
}

pub enum Words {
    Pšenica, //wheat
    Jęčmenj, //barley
    Ržito,   //Rye
    Grozd́je, //grape
    Jablȯko, //apple
    Ježina,  //blackbeery
    Kųpina,  //bramble
}

pub enum GrainType {}

pub enum BerryType {}

pub trait Alcoholizable {}
pub trait Tree {}
pub trait Grass {}
pub trait Fruit {}
pub trait Berry {}

impl Alcoholizable for BerryType {}
impl Alcoholizable for GrainType {}
