use crate::*;
#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum MeleeWeaponType {
    Nož,
    Sěkyra,
    Kyj,
    Meč,
    Kopje,
    Cepj,

    Nagajka,
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum RangedWeaponType {
    Lųk,
    Proca,
    Prak,
    Prašča,
    Šlojder,
    Kuša,
    Samostrěl,
    Kameni,
    Arbalet,
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum StoneType {
    Granit,
    Kremenj,
    Rubin,
    Mramor,
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum MetalType {
    Bronza,
    Zlåto,
    Železo,
    Srebro,
    Medj,
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum WoodType {
    Brest,
    Jasenj,
    Lipa,
    Jablanj,
    Kalina,
    Jalovec,
    Brek,
    Kaštan,
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum GrassType {
    Trava,
    Kovylj, //needle grass
    Burjan, // high grass
    Kanabis,
    Jasenėc,
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum BushType {
    Klubnika,
    Jagoda,     //needle grass
    Zemljanika, // high grass
    Ježina,
    Kųpina,
    Brusnica,
    Malina,
    Kljukva,
    Črnica,
    Žuravina,
    Bȯzina,
}

#[derive(Clone, Debug, Display, PartialEq, EnumIter, EnumCount, FromRepr)]
pub enum MammalType {
    Los,
    Jelenj,
    Krava,
    Pes,
    Tigr,
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

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum AnimalPartType {
    Meat,
    Feather,
    Skin,
    Hair,
    Bone,
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum ShoulderClothingType {
    Šal,

    Šarf,
    Šátek,
    Pléť, //big piece of cloth

    Ruta,      //scarf shawl
    Palantin,  //scarf
    Ogrinjalo, //cape
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum HeadClothingType {
    Kosynka, //headscarf
    Platok,  //headscarf

    Marama, //headscarf
    Voalj,  //veil
    Závoj,  //veil

    Šapka,
    Šljem,
    Kapela,
    Kapuc, //kapushon hood
    Beretka,
    Bandana,
    Vual,
    Klobuk,
    Šešir, //wide brim hat
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum TorsoClothingType {
    Košula, // Shirt
    Tunika, // Tunic
    Halja,  // Dress
    Bluza,  // Blouse
    Majica, // T-shirt
    Kofta,  // Sweater/Cardigan

    Kabanica, // Raincoat
    Kožuh,    // Fur coat
    Vesta,    // Vest

    Koĺčuga,
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum LegsClothingType {
    Pantalony, // Pants
    Hlače,     // Trousers
    Suknja,    // Skirt

    Šarovary,  // Baggy pants
    Spodnjice, // Underpants
}

#[derive(Clone, Debug, Display, PartialEq, EnumCount, FromRepr)]
pub enum MebeljType {
    Stěna,
    Stol,
    Stul,
    Skrinja, //sunduk
    Dvėrj,
    Vråta,
    Vaza,
    Škaf,
}
