use crate::{*, typeenums::Container};



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
    fn item_symbol(&self) -> &'static str ;
   
}


impl ItemTrait for Weapon {
   fn weight(&self) -> i64 {
       match self {
        Weapon::Sword => 1

       }
   }
   fn item_symbol(&self) -> &'static str {
       "/"
   }
}

impl ItemTrait for Container {
    fn weight(&self) -> i64 {
        match self {
         Container::Bag => 1
 
        }
    }
    fn item_symbol(&self) -> &'static str {
        "b"
    }
 }
 

impl ContainerTrait for Container {
    fn space(&self) -> i64 {
        match self {
         Container::Bag => 10
 
        }
    }
 }