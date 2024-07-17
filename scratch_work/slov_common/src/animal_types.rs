use crate::*;

// fishes, amphibians, reptiles, birds, and mammals.
#[derive(Clone, Debug, Display, PartialEq)]
pub enum ImperiumNaturae {
    Regio,
    Regnum,
    Phylum,
    Classis,
    Ordo,
    Familia,
    Genus,
    Species,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Regna {
    Animalia,
    Vegetabilia,
    Lapides,
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Animalia {
    Mammalia,
    Aves,
    Amphibia,
    Pisces,
    Insecta,
    Vermes,
}
