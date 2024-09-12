use strum::Display;

#[derive(Display, PartialEq, Clone)]
pub enum AlcoholicDrink<T: Alcoholizable> {
    Pivo(T),
    Vino(T),
    Vodka(T),
    Samogon(T),
}
#[derive(Display, PartialEq, Clone)]
pub enum GrainType {
    Pšenica, //wheat
    Jęčmenj, //barley
    Ržito,   //Rye
}
#[derive(Display, PartialEq, Clone)]
pub enum BerryType {
    Grozd́je,
    Jablȯko,
    Ježina,
    Kųpina,
}
#[derive(Display, PartialEq, Clone)]
pub enum RootVegetableType {
    Kartofelj,
}

pub trait Alcoholizable {}
pub trait Tree {}
pub trait Grass {}
pub trait Fruit {}
pub trait Berry {}

impl Alcoholizable for BerryType {}
impl Alcoholizable for GrainType {}
