use crate::*;


// Define the Material trait if not already defined
pub trait Material {
    // Define the methods and associated functions for the trait
    fn mat_color(&self) -> RatColor;
}


impl Material for Tree {
    fn mat_color(&self) -> RatColor {
        self.color()
    }
}