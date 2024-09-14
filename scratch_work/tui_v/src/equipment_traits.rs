use crate::*;

pub trait ItemTrait {
    fn item_symbol(&self) -> String;
    fn item_name(&self) -> String;
}

impl ItemTrait for AnimalType {
    fn item_symbol(&self) -> String {
        let nameik = self.item_name();
        let a = first_char(&nameik);
        String::from(a)
    }
    fn item_name(&self) -> String {
        format!("{self}")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BodyPart {
    Head,
    HeadAccesory,
    Ring,
    Bracelet,
    Necklace,
    Torso,

    TorsoOver,
    Cloak,
    Waist,
    Legs,
    Feet,
    FeetOver,
}

pub trait Armor {
    fn body_part_covered(&self) -> BodyPart;
    fn defense_value(&self) -> i32;
}

impl Armor for Clothing {
    fn body_part_covered(&self) -> BodyPart {
        match self {
            Clothing::Toga => BodyPart::Cloak,
            Clothing::Helma => BodyPart::Head,
            Clothing::Narųčka | Clothing::Braslet => BodyPart::Bracelet, // Bracelet/Accessory
            Clothing::Koljce | Clothing::Pŕstenj => BodyPart::Ring,      // Ring/Accessory
            Clothing::Našijnik | Clothing::Ogrlica | Clothing::Monisto => BodyPart::Necklace, // Necklace/Beads/Accessory
            Clothing::Čepec | Clothing::Kapjušon | Clothing::Šapka | Clothing::Klobuk => {
                BodyPart::HeadAccesory
            } // Head Accessory
            Clothing::Plašč => BodyPart::Cloak,      // Cloak
            Clothing::Košulja => BodyPart::Torso,    // Shirt
            Clothing::Riza => BodyPart::TorsoOver,   // Cassock/Robe
            Clothing::Sandaly => BodyPart::FeetOver, // Sandals
            Clothing::Škarpetky | Clothing::Ponožky => BodyPart::Feet, // Socks
            Clothing::Šlěm => BodyPart::Head,        // Helmet
            Clothing::Suknja | Clothing::Frak => BodyPart::Legs, // Dress/Tailcoat
            Clothing::Koljčuga => BodyPart::Torso,   // Chainmail
            Clothing::Brȯnja | Clothing::Pancyŕ => BodyPart::TorsoOver, // Armor
            Clothing::Plåtno | Clothing::Tkanina => BodyPart::Cloak, // Cloth
            Clothing::Vualj => BodyPart::HeadAccesory, // Veil/Face
            Clothing::Šal => BodyPart::HeadAccesory, // Shawl/Head Accessory
            Clothing::Kožuh => BodyPart::TorsoOver,  // Fur coat
            Clothing::Pojas => BodyPart::Waist,      // Belt/Girdle
            Clothing::Šata => BodyPart::TorsoOver,   // Gown
            Clothing::Valenky => BodyPart::FeetOver,
        }
    }

    fn defense_value(&self) -> i32 {
        match self {
            Clothing::Toga => 5,                        // Basic defense for a cloak
            Clothing::Helma => 8,                       // Decent protection from a helmet
            Clothing::Narųčka | Clothing::Braslet => 1, // Minimal defense from a bracelet
            Clothing::Koljce | Clothing::Pŕstenj => 1,  // Minimal defense from a ring
            Clothing::Našijnik | Clothing::Ogrlica | Clothing::Monisto => 1, // Minimal defense from a necklace
            Clothing::Čepec => 2,    // Basic protection from a bonnet
            Clothing::Kapjušon => 3, // Slightly more protection from a hood
            Clothing::Plašč => 4,    // Basic defense from a cloak
            Clothing::Košulja => 2,  // Light protection from a shirt
            Clothing::Riza => 5,     // Moderate defense from a cassock/robe
            Clothing::Sandaly => 1,  // Minimal protection for feet
            Clothing::Škarpetky | Clothing::Ponožky => 0, // No defense from socks
            Clothing::Šlěm => 10,    // Strong protection from a helmet
            Clothing::Šapka => 2,    // Light defense from a cap
            Clothing::Klobuk => 2,   // Light defense from a hat
            Clothing::Suknja => 2,   // Light defense from a dress
            Clothing::Frak => 3,     // Moderate protection from a tailcoat
            Clothing::Koljčuga => 12, // Strong defense from chainmail
            Clothing::Brȯnja | Clothing::Pancyŕ => 15, // High defense from armor
            Clothing::Plåtno | Clothing::Tkanina => 1, // Minimal defense from cloth
            Clothing::Vualj => 0,    // No defense from a veil
            Clothing::Šal => 1,      // Minimal defense from a shawl
            Clothing::Kožuh => 6,    // Good defense from a fur coat
            Clothing::Pojas => 1,    // Minimal defense from a belt/girdle
            Clothing::Šata => 4,     // Moderate defense from a gown
            Clothing::Valenky => 2,
        }
    }
}

impl ItemTrait for ItemType {
    fn item_symbol(&self) -> String {
        let nameik = self.item_name();
        let a = first_char(&nameik);
        a.to_lowercase()
    }
    fn item_name(&self) -> String {
        match self {
            ItemType::Weapon(wep) => {
                format!("{wep}")
            }

            ItemType::Clothing(cloth) => {
                format!("{cloth}")
            }
            ItemType::Consumable(cons) => {
                format!("{cons}")
            }

            ItemType::Ammo(amm) => match amm {
                Ammo::Kulja(x) | Ammo::Strěla(x) | Ammo::Oščěp(x) | Ammo::Drotik(x) => {
                    format!("{amm}({x})")
                }
            },
        }
    }
}

pub enum DamageType {
    Sharp,
    Blunt,
}
impl Weapon {
    pub fn handedness(&self) -> i32 {
        match self {
            Weapon::Meč => 1,
            Weapon::Ščit => 1,
            Weapon::Bulava => 1,
            Weapon::Žezlo => 1,
            Weapon::Posoh => 2,
            Weapon::Kopje => 2,
            Weapon::Nož => 1,
            Weapon::Kyj => 1,
            Weapon::Prųt => 1,
            Weapon::Sŕp => 1,
            Weapon::Cěp => 2,
            Weapon::Lancuh => 2,
            Weapon::Kosa => 2,
            Weapon::Lopata => 2,
            Weapon::Vidla => 2,
            Weapon::Sěkyra => 1,
            Weapon::Bič => 2,
            Weapon::Sablja => 1,
        }
    }
    pub fn weapon_length(&self) -> i32 {
        match self {
            Weapon::Meč => 5,
            Weapon::Bulava => 4,
            Weapon::Ščit => 15,
            Weapon::Žezlo => 3,
            Weapon::Posoh => 7,
            Weapon::Kopje => 8,
            Weapon::Nož => 1,
            Weapon::Kyj => 3,
            Weapon::Prųt => 4,
            Weapon::Sŕp => 2,
            Weapon::Cěp => 8,
            Weapon::Lancuh => 7,
            Weapon::Kosa => 9,
            Weapon::Lopata => 7,
            Weapon::Vidla => 7,
            Weapon::Sěkyra => 4,
            Weapon::Bič => 6,
            Weapon::Sablja => 7,
        }
    }
}
pub trait WeaponTrait {
    fn damage(&self) -> i32;
    fn damage_type(&self) -> DamageType;
}

impl WeaponTrait for Weapon {
    fn damage(&self) -> i32 {
        match self {
            Weapon::Meč => 20,
            Weapon::Bulava => 20,
            Weapon::Ščit => 1,
            Weapon::Žezlo => 15,
            Weapon::Posoh => 10,
            Weapon::Kopje => 20,
            Weapon::Nož => 10,
            Weapon::Kyj => 5,
            Weapon::Prųt => 10,
            Weapon::Sŕp => 15,
            Weapon::Cěp => 15,
            Weapon::Lancuh => 20,
            Weapon::Kosa => 25,
            Weapon::Lopata => 12,
            Weapon::Vidla => 18,
            Weapon::Sěkyra => 17,
            Weapon::Bič => 12,
            Weapon::Sablja => 24,
        }
    }
    fn damage_type(&self) -> DamageType {
        match self {
            Weapon::Meč => DamageType::Sharp,
            Weapon::Bulava => DamageType::Blunt,
            Weapon::Ščit => DamageType::Blunt,
            Weapon::Žezlo => DamageType::Blunt,
            Weapon::Posoh => DamageType::Blunt,
            Weapon::Kopje => DamageType::Sharp,
            Weapon::Nož => DamageType::Sharp,
            Weapon::Kyj => DamageType::Blunt,
            Weapon::Prųt => DamageType::Blunt,
            Weapon::Sŕp => DamageType::Sharp,
            Weapon::Cěp => DamageType::Blunt,
            Weapon::Lancuh => DamageType::Blunt,
            Weapon::Kosa => DamageType::Sharp,
            Weapon::Lopata => DamageType::Sharp,
            Weapon::Vidla => DamageType::Sharp,
            Weapon::Sěkyra => DamageType::Sharp,
            Weapon::Bič => DamageType::Blunt,
            Weapon::Sablja => DamageType::Sharp,
        }
    }
}
impl RangedWeapon {
    pub fn ideal_range(&self) -> CoordinateUnit {
        match self {
            RangedWeapon::Lųk => 10,
            RangedWeapon::Pråšča => 7,
            RangedWeapon::Drotik => 3,
            RangedWeapon::Oščěp => 5,
        }
    }
}

impl WeaponTrait for RangedWeapon {
    fn damage(&self) -> i32 {
        match self {
            RangedWeapon::Lųk => 30,
            RangedWeapon::Pråšča => 20,
            RangedWeapon::Drotik => 10,
            RangedWeapon::Oščěp => 40,
        }
    }
    fn damage_type(&self) -> DamageType {
        match self {
            RangedWeapon::Lųk => DamageType::Sharp,
            RangedWeapon::Pråšča => DamageType::Blunt,
            RangedWeapon::Drotik => DamageType::Sharp,
            RangedWeapon::Oščěp => DamageType::Sharp,
        }
    }
}
