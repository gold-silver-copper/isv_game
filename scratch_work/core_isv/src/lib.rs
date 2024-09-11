pub enum DrinkType {
    Pivo((Box<dyn Alcoholizable>)),
    Vino((Box<dyn Alcoholizable>)),
    Vodka((Box<dyn Alcoholizable>)),
    Samogon((Box<dyn Alcoholizable>)),
}

pub enum GrainType {
    Pšenica, //wheat
    Jęčmenj, //barley
    Ržito,   //Rye
}

pub enum BerryType {
    Grozd́je,
    Jablȯko,
    Ježina,
    Kųpina,
}

pub trait Alcoholizable {}

impl Alcoholizable for BerryType {}
impl Alcoholizable for GrainType {}
