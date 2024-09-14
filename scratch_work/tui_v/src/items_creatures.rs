use crate::*;

#[derive(Display, PartialEq, Clone)]
pub enum EntityType {
    Human(Profession),
    Item(ItemType),
    Animal(AnimalType),
}
#[derive(Display, PartialEq, Clone)]
pub enum Profession {
    Sluga,
    Žebrak,       //beggar
    Prośak,       //beggar
    Vęzėnj,       //prisoner
    Zaključiĺnik, //prisoner
    Ubijca,       //killer
    Rycaŕ,        //knight
    Vitęź,        //knight
    Varvar,       //barbarian
    Trgovec,
    Kupec,
    Rybak,      // fisherman
    Kovač,      // blacksmith
    Zemjedělec, // farmer
    Lovec,      // hunter
    Mųdrec,     // sage/wizard
    Čarovnik,   // sorcerer/mage
    Dezerter,
    Žertvaŕ, // sacrificial priest
    Volhvaŕ, // sorcerer-priest
}

impl Profession {
    pub fn random_weapon(&self) -> Vec<Weapon> {
        match self {
            Profession::Sluga => vec![Weapon::Lopata, Weapon::Prųt], // basic tools
            Profession::Žebrak => vec![Weapon::Kyj],                 // weak
            Profession::Prośak => vec![Weapon::Prųt],                // beggar
            Profession::Vęzėnj => vec![Weapon::Sŕp],                 // prisoner
            Profession::Zaključiĺnik => vec![Weapon::Nož],           // prisoner
            Profession::Ubijca => vec![Weapon::Nož, Weapon::Kyj],    // killer
            Profession::Rycaŕ => vec![Weapon::Meč, Weapon::Ščit],    // knight
            Profession::Vitęź => vec![Weapon::Sablja, Weapon::Ščit], // knight
            Profession::Varvar => vec![Weapon::Sěkyra, Weapon::Kyj], // barbarian
            Profession::Trgovec => vec![Weapon::Nož],                // merchant
            Profession::Kupec => vec![Weapon::Nož],                  // merchant
            Profession::Rybak => vec![Weapon::Sŕp, Weapon::Vidla],   // fisherman
            Profession::Kovač => vec![Weapon::Sěkyra, Weapon::Meč],  // blacksmith
            Profession::Zemjedělec => vec![Weapon::Vidla, Weapon::Lopata], // farmer
            Profession::Lovec => vec![Weapon::Kopje, Weapon::Nož],   // hunter
            Profession::Mųdrec => vec![Weapon::Žezlo, Weapon::Posoh], // sage/wizard
            Profession::Čarovnik => vec![Weapon::Posoh, Weapon::Bič], // sorcerer/mage
            Profession::Dezerter => vec![Weapon::Cěp, Weapon::Lancuh], // deserter
            Profession::Žertvaŕ => vec![Weapon::Bič, Weapon::Meč],   // sacrificial priest
            Profession::Volhvaŕ => vec![Weapon::Žezlo, Weapon::Bulava], // sorcerer-priest
        }
    }
    pub fn random_equip(&self) -> Vec<Clothing> {
        match self {
            Profession::Sluga => vec![Clothing::Plašč, Clothing::Suknja], // basic
            Profession::Žebrak => vec![Clothing::Plašč],                  // weak
            Profession::Prośak => vec![Clothing::Košulja, Clothing::Šal], // beggar
            Profession::Vęzėnj => vec![Clothing::Pojas, Clothing::Riza],  // prisoner
            Profession::Zaključiĺnik => vec![Clothing::Toga, Clothing::Pojas], // prisoner
            Profession::Ubijca => vec![Clothing::Narųčka, Clothing::Helma, Clothing::Pŕstenj], // killer
            Profession::Rycaŕ => vec![Clothing::Šlěm, Clothing::Koljčuga, Clothing::Pancyŕ], // knight
            Profession::Vitęź => vec![Clothing::Helma, Clothing::Brȯnja, Clothing::Koljce], // knight
            Profession::Varvar => vec![Clothing::Kožuh, Clothing::Pojas, Clothing::Helma], // barbarian
            Profession::Trgovec => vec![Clothing::Frak, Clothing::Našijnik], // merchant
            Profession::Kupec => vec![Clothing::Frak, Clothing::Ogrlica],    // merchant
            Profession::Rybak => vec![Clothing::Šapka, Clothing::Košulja],   // fisherman
            Profession::Kovač => vec![Clothing::Šlěm, Clothing::Narųčka],    // blacksmith
            Profession::Zemjedělec => vec![Clothing::Plåtno, Clothing::Valenky], // farmer
            Profession::Lovec => vec![Clothing::Šal, Clothing::Sandaly, Clothing::Kapjušon], // hunter
            Profession::Mųdrec => vec![Clothing::Toga, Clothing::Monisto, Clothing::Šata], // sage/wizard
            Profession::Čarovnik => vec![Clothing::Toga, Clothing::Našijnik, Clothing::Vualj], // sorcerer/mage
            Profession::Dezerter => vec![Clothing::Pojas, Clothing::Plašč], // deserter
            Profession::Žertvaŕ => vec![Clothing::Monisto, Clothing::Čepec, Clothing::Košulja], // sacrificial priest
            Profession::Volhvaŕ => vec![Clothing::Šata, Clothing::Našijnik, Clothing::Ogrlica], // sorcerer-priest
        }
    }

    pub fn skill_level(&self) -> i64 {
        match self {
            Profession::Sluga => 20,        // basic
            Profession::Žebrak => 10,       // weak
            Profession::Prośak => 15,       // very weak
            Profession::Vęzėnj => 25,       // low strength
            Profession::Zaključiĺnik => 30, // medium-low
            Profession::Ubijca => 40,       // rather strong
            Profession::Rycaŕ => 100,       // very strong
            Profession::Vitęź => 95,        // nearly as strong
            Profession::Varvar => 80,       // strong
            Profession::Trgovec => 35,      // moderate
            Profession::Kupec => 35,        // moderate
            Profession::Rybak => 20,        // basic
            Profession::Kovač => 50,        // skilled
            Profession::Zemjedělec => 25,   // basic to moderate
            Profession::Lovec => 60,        // experienced
            Profession::Mųdrec => 70,       // wise and strong
            Profession::Čarovnik => 85,     // powerful mage
            Profession::Dezerter => 30,     // untrustworthy but capable
            Profession::Žertvaŕ => 50,      // experienced priest
            Profession::Volhvaŕ => 75,      // highly skilled sorcerer-priest
        }
    }
}

#[derive(Display, PartialEq, Clone)]
pub enum AnimalType {
    Loś,
    Běs,
    Djavȯl,
    Jelenj,
    Krava,
    Pes,
    Tigr,
    Gad,
    Jaščer,
    Iguana,
    Vųž,
    Žȯlv,
    Sova,
    Vrabec,
    Vran,
    Gavran,
    Kos,
    Gųsę,
    Gųś,
    Vȯlk,
    Vȯlkolak,
    Vȯlkodav,
}

impl AnimalType {
    pub fn max_stat(&self) -> i64 {
        match self {
            AnimalType::Loś => 50, // Moose (Los) are large and strong animals
            AnimalType::Gųś => 20,
            AnimalType::Djavȯl => 300,
            AnimalType::Běs => 150,
            AnimalType::Jelenj => 45, // Deer (Jelenj) are strong but not as powerful as moose
            AnimalType::Krava => 40,  // Cow (Krava) is strong, but slower and less agile
            AnimalType::Pes => 30,    // Dog (Pes) has moderate strength, with agility and speed
            AnimalType::Vȯlk => 40,
            AnimalType::Vȯlkolak => 80,
            AnimalType::Vȯlkodav => 35,
            AnimalType::Tigr => 60, // Tiger (Tigr) is incredibly strong and agile
            AnimalType::Gad => 15,  // Reptile (Gad) has limited strength
            AnimalType::Jaščer => 10, // Lizard (Jaščer) is small and not very strong
            AnimalType::Iguana => 12, // Iguana has slightly more strength than smaller lizards
            AnimalType::Vųž => 8,   // Snake (Vųž) is not very strong, but has other abilities
            AnimalType::Žȯlv => 20, // Tortoise (Žȯlv) is slow but has some physical strength
            AnimalType::Sova => 10, // Owl (Sova) has low physical strength but good agility
            AnimalType::Vrabec => 5, // Sparrow (Vrabec) is small and weak
            AnimalType::Vran => 15, // Crow (Vran) has moderate strength for a bird
            AnimalType::Gavran => 18, // Raven (Gavran) is stronger than the crow
            AnimalType::Kos => 8,   // Blackbird (Kos) is small and not strong
            AnimalType::Gųsę => 25, // Goose (Gųsę) is surprisingly strong for its size
        }
    }
}

impl EntityType {
    pub fn is_attackable(&self) -> bool {
        match self {
            EntityType::Human(_) => true,
            EntityType::Animal(_) => true,
            EntityType::Item(_) => false,
        }
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
    pub fn consume_verb(&self) -> &'static str {
        match self {
            Consumable::Lěkaŕstvo => "vypivati", // To drink (for medicine)
            Consumable::Hlěb => "sjedati",       // To eat (for bread)
            Consumable::Pečivo => "sjedati",     // To eat (for baked goods)
            Consumable::Pŕnik => "sjedati",      // To eat (for gingerbread)
            Consumable::Polivka | Consumable::Supa => "vypivati", // To drink (for soup)
            Consumable::Brusnica
            | Consumable::Morva
            | Consumable::Malina
            | Consumable::Kljukva
            | Consumable::Ježina => "sjedati", // To eat (for berries)
            Consumable::Rěpa | Consumable::Cvekla | Consumable::Redkȯvka | Consumable::Kapusta => {
                "sjedati"
            } // To eat (for vegetables)
            Consumable::Jablȯko => "sjedati",    // To eat (for apples)
            Consumable::Vino => "vypivati",      // To drink (for wine)
            Consumable::Samogon => "vypivati",   // To drink (for strong alcohol)
            Consumable::Pivo => "vypivati",      // To drink (for beer)
        }
    }

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
    Ščit,
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
    Škarpetky,
    Ponožky,
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
