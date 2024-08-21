use crate::*;


// Define the Material trait if not already defined
pub trait Material {
    // Define the methods and associated functions for the trait
    fn meow(&self) -> String;
}


impl Material for Tree {
    fn meow(&self) -> String {
      String::from("meow")
    }
}

impl Material for Metal {
    fn meow(&self) -> String {
      String::from("meow")
    }
}