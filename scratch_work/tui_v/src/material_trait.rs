use crate::*;


// Define the Material trait if not already defined
pub trait Material {
    // Define the methods and associated functions for the trait
    fn describe(&self) -> String;
}


impl Material for Tree {
    fn describe(&self) -> String {
        format!("This is a solid material called lol")
    }
}