use crate::{typeenums::Container, *};

pub trait Wieldable {
    fn damage(&self) -> i64;
}
pub trait Wearable {
    fn damage(&self) -> i64;
}
pub trait ContainerTrait {
    fn space(&self) -> i64;
}

pub trait ItemTrait {
    fn weight(&self) -> i64;
    fn item_symbol(&self) -> &'static str;
    fn item_name(&self) -> String;
}

impl ItemTrait for ItemType {
    fn weight(&self) -> i64 {
        match self {
            ItemType::Weapon(wep) => 1,
            ItemType::Clothing(cloth) => 1,
            ItemType::Container(cont) => 1,
        }
    }
    fn item_symbol(&self) -> &'static str {
        match self {
            ItemType::Weapon(wep) => "/",
            ItemType::Container(cont) => "b",
            ItemType::Clothing(cloth) => "t",
        }
    }
    fn item_name(&self) -> String {
        match self {
            ItemType::Weapon(wep) => {
                format!("{wep}")
            }
            ItemType::Container(cont) => {
                format!("{cont}")
            }
            ItemType::Clothing(cloth) => {
                format!("{cloth}")
            }
        }
    }
}

impl ContainerTrait for Container {
    fn space(&self) -> i64 {
        match self {
            Container::Bag => 10,
        }
    }
}
