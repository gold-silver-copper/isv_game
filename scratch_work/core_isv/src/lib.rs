use strum::Display;

//#[derive(Display, PartialEq, Clone)]
pub enum AlcoholicDrink {
    Pivo(Plant),
    Vino(Plant),
    Vodka(Plant),
    Samogon(Plant),
}

pub enum Clothes {
    LeftGlove(FlexibleMaterial),
}

pub enum FlexibleMaterial {
    Leather(Animal),
    Hair(Animal),
    Cloth(Plant),
}

pub enum Stone {
    Quartz,
}
pub enum Metal {
    Bronze,
}

pub enum Utensil {
    Goblet(SolidMaterial),
}

pub enum SolidMaterial {
    Stone(Stone),
    Metal(Metal),
    Wood(Plant),
}

pub enum Plant {
    Apple,
    Nut,
}
pub enum LeafType {
    Broad,
    Needle,
}
pub enum FruitType {
    Fruit,
    Nut,
    Berry,
    Pod,
    Grain,
    Cone,
    Seed,
    Spore,
}
pub enum StemType {
    Tree,
    Shrub,
    Grass,
    Herb,
    Climbing,
    Creeping,
    Aquatic,
}
pub enum RootType {
    Normal,
    Bulb,
    Tubers,
}
impl Plant {
    fn fruit_production(&self) -> FruitType {
        match self {
            Plant::Apple => FruitType::Fruit,
            Plant::Nut => FruitType::Nut,
        }
    }
}

pub enum Animal {
    Cow,
}

pub enum AnimalProduct {
    Skin(Animal),
    Milk(Animal),
}
