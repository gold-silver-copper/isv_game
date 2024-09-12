use strum::Display;
/*
#[derive(Display, PartialEq, Clone)]
pub enum AlcoholicDrink<T: Alcoholizable> {
    Pivo(T),
    Vino(T),
    Vodka(T),
    Samogon(T),
}
pub pub trait Alcoholizable {}
#[derive(Display, PartialEq, Clone)]
pub enum CerealGrain {
    Pšenica, //wheat
    Jęčmenj, //barley
    Ržito,   //Rye
}
#[derive(Display, PartialEq, Clone)]
pub enum BerryType {
    Grozd́je,
    Jablȯko,
    Ježina,
    Kųpina,
}
#[derive(Display, PartialEq, Clone)]
pub enum RootVegetableType {
    Kartofelj,
} */

// pub trait for any ingredient that can be used in food production
pub trait Ingredient {
    fn name(&self) -> &'static str;
}

// Grain-specific pub trait
pub trait Grain: Ingredient {
    fn use_for_bread(&self) -> String;
    fn use_for_beer(&self) -> String;
}

// Fruit-specific pub trait
pub trait Fruit: Ingredient {
    fn use_for_wine(&self) -> String;
    fn use_for_pastries(&self, combined_with_grain: &impl Grain) -> String;
}

// Vegetable-specific pub trait
pub trait Vegetable: Ingredient {
    fn use_for_soup(&self) -> String;
}

// Tree-specific pub trait for wooden products
pub trait Tree: Ingredient {
    fn use_for_wooden_bowl(&self) -> String;
}

// Specific Grain: Wheat
pub struct Wheat;

impl Ingredient for Wheat {
    fn name(&self) -> &'static str {
        "Wheat"
    }
}

impl Grain for Wheat {
    fn use_for_bread(&self) -> String {
        format!("{} is used to make bread", self.name())
    }

    fn use_for_beer(&self) -> String {
        format!("{} is used to make beer", self.name())
    }
}

// Specific Grain: Barley
pub struct Barley;

impl Ingredient for Barley {
    fn name(&self) -> &'static str {
        "Barley"
    }
}

impl Grain for Barley {
    fn use_for_bread(&self) -> String {
        format!("{} is used to make bread", self.name())
    }

    fn use_for_beer(&self) -> String {
        format!("{} is used to make beer", self.name())
    }
}

// Specific Fruit: Grape
pub struct Grape;

impl Ingredient for Grape {
    fn name(&self) -> &'static str {
        "Grape"
    }
}

impl Fruit for Grape {
    fn use_for_wine(&self) -> String {
        format!("{} is used to make wine", self.name())
    }

    fn use_for_pastries(&self, grain: &impl Grain) -> String {
        format!(
            "{} is combined with {} to make a pastry",
            self.name(),
            grain.name()
        )
    }
}

// Specific Fruit: Apple
pub struct Apple;

impl Ingredient for Apple {
    fn name(&self) -> &'static str {
        "Apple"
    }
}

impl Fruit for Apple {
    fn use_for_wine(&self) -> String {
        format!("{} is used to make cider", self.name())
    }

    fn use_for_pastries(&self, grain: &impl Grain) -> String {
        format!(
            "{} is combined with {} to make a pastry",
            self.name(),
            grain.name()
        )
    }
}

// Specific Vegetable: Carrot
pub struct Carrot;

impl Ingredient for Carrot {
    fn name(&self) -> &'static str {
        "Carrot"
    }
}

impl Vegetable for Carrot {
    fn use_for_soup(&self) -> String {
        format!("{} is used to make vegetable soup", self.name())
    }
}

// Specific Vegetable: Onion
pub struct Onion;

impl Ingredient for Onion {
    fn name(&self) -> &'static str {
        "Onion"
    }
}

impl Vegetable for Onion {
    fn use_for_soup(&self) -> String {
        format!("{} is used to make vegetable soup", self.name())
    }
}
// Specific Tree: Oak
pub struct Oak;

impl Ingredient for Oak {
    fn name(&self) -> &'static str {
        "Oak"
    }
}

impl Tree for Oak {
    fn use_for_wooden_bowl(&self) -> String {
        format!("{} is used to carve a wooden bowl", self.name())
    }
}

// Specific Tree: Maple
pub struct Maple;

impl Ingredient for Maple {
    fn name(&self) -> &'static str {
        "Maple"
    }
}

impl Tree for Maple {
    fn use_for_wooden_bowl(&self) -> String {
        format!("{} is used to carve a wooden bowl", self.name())
    }
}
