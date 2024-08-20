use crate::*;


pub trait Blockable {
    fn blocks_movement(&self) -> bool;
    fn blocks_vision(&self) -> bool;
}


impl Blockable for Furniture {
    fn blocks_movement(&self) -> bool {
        match &self {
            Furniture::Wall(_) => true,
            _ => false,
        }
    }

    fn blocks_vision(&self) -> bool {
        match &self {
            Furniture::Wall(_) => true,
            _ => false,
        }
    }
}

impl Blockable for EntityType {
    fn blocks_movement(&self) -> bool {
        match &self {
            _ => true,  // Default behavior, all entities block movement
        }
    }

    fn blocks_vision(&self) -> bool {
        match &self {
            _ => false,  // Default behavior, entities do not block vision unless otherwise specified
        }
    }
}
