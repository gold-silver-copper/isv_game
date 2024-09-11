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
    Consumable(Consumable),

    Ammo(Ammo),
    Clothing(Clothing),
}
#[derive(Display, PartialEq, Clone)]
pub enum Consumable {
    Lěkaŕstvo,
    Hlěb,
    Pečivo,
    Pŕnik,
    Polivka,
    Supa,
    Brusnica,
    Morva,
    Malina,
    Kljukva,
    Ježina,
    Rěpa,
    Cvekla,
    Redkȯvka,
    Kapusta,
    Jablȯko,
    Vino,
    Samogon,

    Pivo,
}

impl Consumable {
    pub fn health_effect(&self) -> i32 {
        match self {
            Consumable::Lěkaŕstvo => 20, // Medicine provides the highest health restoration
            Consumable::Hlěb => 8,       // Bread provides decent health restoration
            Consumable::Pečivo => 5,     // Baked goods restore some health
            Consumable::Pŕnik => 6,      // Gingerbread restores some health
            Consumable::Polivka | Consumable::Supa => 10, // Soups provide moderate health restoration
            Consumable::Brusnica
            | Consumable::Morva
            | Consumable::Malina
            | Consumable::Kljukva
            | Consumable::Ježina => 4, // Berries provide light health restoration
            Consumable::Rěpa | Consumable::Cvekla | Consumable::Redkȯvka | Consumable::Kapusta => {
                6
            } // Vegetables provide light to moderate health restoration
            Consumable::Jablȯko => 5,                     // Apples restore a small amount of health
            Consumable::Vino => 5,                        // Wine provides some health restoration
            Consumable::Samogon => 10,                    // Strong alcohol can damage health
            Consumable::Pivo => 5,                        // Beer restores a little health
        }
    }

    pub fn strength_effect(&self) -> i32 {
        match self {
            Consumable::Lěkaŕstvo => 0, // Medicine does not affect strength
            Consumable::Hlěb => 1,      // Bread provides a small strength boost
            Consumable::Pečivo => 0,    // Baked goods don't affect strength
            Consumable::Pŕnik => 0,     // Gingerbread does not boost strength
            Consumable::Polivka | Consumable::Supa => 1, // Soups slightly increase strength
            Consumable::Brusnica
            | Consumable::Morva
            | Consumable::Malina
            | Consumable::Kljukva
            | Consumable::Ježina => 0, // Berries do not affect strength
            Consumable::Rěpa | Consumable::Cvekla | Consumable::Redkȯvka | Consumable::Kapusta => {
                1
            } // Vegetables provide a slight strength boost
            Consumable::Jablȯko => 0,   // Apples don't affect strength
            Consumable::Vino => 1,      // Wine provides a small strength boost
            Consumable::Samogon => 2,   // Strong alcohol provides a higher strength boost
            Consumable::Pivo => 1,      // Beer provides a small strength boost
        }
    }

    pub fn int_effect(&self) -> i32 {
        match self {
            Consumable::Lěkaŕstvo => 0, // Medicine does not affect intelligence
            Consumable::Hlěb => 0,      // Bread doesn't affect intelligence
            Consumable::Pečivo => 0,    // Baked goods don't affect intelligence
            Consumable::Pŕnik => 0,     // Gingerbread does not boost intelligence
            Consumable::Polivka | Consumable::Supa => 0, // Soups do not affect intelligence
            Consumable::Brusnica
            | Consumable::Morva
            | Consumable::Malina
            | Consumable::Kljukva
            | Consumable::Ježina => 1, // Berries slightly boost intelligence
            Consumable::Rěpa | Consumable::Cvekla | Consumable::Redkȯvka | Consumable::Kapusta => {
                0
            } // Vegetables do not affect intelligence
            Consumable::Jablȯko => 1,   // Apples provide a small intelligence boost
            Consumable::Vino => 1,      // Wine boosts intelligence slightly
            Consumable::Samogon => 0,   // Strong alcohol reduces intelligence
            Consumable::Pivo => 0,      // Beer does not affect intelligence
        }
    }

    pub fn speed_effect(&self) -> i32 {
        match self {
            Consumable::Lěkaŕstvo => 0, // Medicine does not affect speed
            Consumable::Hlěb => 0,      // Bread doesn't affect speed
            Consumable::Pečivo => 0,    // Baked goods don't affect speed
            Consumable::Pŕnik => 0,     // Gingerbread does not affect speed
            Consumable::Polivka | Consumable::Supa => 0, // Soups do not affect speed
            Consumable::Brusnica
            | Consumable::Morva
            | Consumable::Malina
            | Consumable::Kljukva
            | Consumable::Ježina => 1, // Berries slightly boost speed
            Consumable::Rěpa | Consumable::Cvekla | Consumable::Redkȯvka | Consumable::Kapusta => {
                0
            } // Vegetables do not affect speed
            Consumable::Jablȯko => 1,   // Apples slightly boost speed
            Consumable::Vino => 0,      // Wine reduces speed slightly
            Consumable::Samogon => 0,   // Strong alcohol reduces speed more
            Consumable::Pivo => 0,      // Beer slightly reduces speed
        }
    }
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
    Meč,
    Bulava,
    Žezlo,
    Posoh,
    Kopje,
    Nož,
    Kyj,
    Prųt,
    Sŕp,
    Cěp,
    Lancuh,
    Kosa,
    Lopata,
    Vidla,
    Sěkyra,
    Bič,
    Sablja,
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
    Kulja(i32),
    Oščěp(i32),
    Strěla(i32),
    Drotik(i32),
}
#[derive(Clone, Debug, PartialEq, Display)]
pub enum Clothing {
    Toga,
    Helma,
    Narųčka,
    Braslet,
    Koljce,
    Pŕstenj,
    Našijnik,
    Ogrlica,
    Monisto,
    Čepec,
    Kapjušon,
    Plašč,
    Košulja,
    Riza,
    Sandaly,
    Škarpetka,
    Ponožka,
    Šlěm,
    Šapka,
    Klobuk,
    Suknja,
    Frak,
    Koljčuga,
    Brȯnja,
    Pancyŕ,
    Plåtno,
    Tkanina,
    Vualj,
    Šal,
    Kožuh,
    Pojas,
    Šata,
    Valenky,
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
