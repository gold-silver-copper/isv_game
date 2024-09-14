use crate::*;

pub trait Blockable {
    fn blocks_movement(&self) -> bool;
    fn blocks_vision(&self) -> bool;
}

impl Blockable for Furniture {
    fn blocks_movement(&self) -> bool {
        match &self {
            Furniture::Wall => true,
            Furniture::Tree => true,
        }
    }

    fn blocks_vision(&self) -> bool {
        match &self {
            Furniture::Wall => true,
            Furniture::Tree => true,
        }
    }
}

impl Blockable for EntityType {
    fn blocks_movement(&self) -> bool {
        match &self {
            EntityType::Human(_) => true, // Default behavior, all entities block movement
            EntityType::Item(_) => false, // Default behavior, all itesm dont block movement
            EntityType::Animal(_) => true,
        }
    }

    fn blocks_vision(&self) -> bool {
        match &self {
            _ => false, // Default behavior, entities do not block vision unless otherwise specified
        }
    }
}
