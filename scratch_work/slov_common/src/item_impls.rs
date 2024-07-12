use crate::*;

impl SolidMaterial {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Metal(x) => x.to_color(),
            Self::Drěvo(x) => x.to_color(),
            Self::Kamenj(x) => x.to_color(),
        }
    }
}

impl FabricMaterial {
    pub fn to_color(&self) -> Color {
        match &self {
            Self::Tkanina(x) => x.to_color(),
            Self::Koža(x) => x.to_color(),

            Self::Lancuh(x) => x.to_color(),
        }
    }
}

impl WoodType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(139, 69, 19),
        }
    }
}

impl MetalType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(170, 169, 173),
        }
    }
}

impl StoneType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(118, 91, 70),
        }
    }
}

impl GrassType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(34, 139, 34),
        }
    }
    pub fn to_displaychar(&self) -> String {
        match &self {
            Self::Trava => "'".into(),
            Self::Kovylj => "\"".into(),
            Self::Burjan => "/".into(),
            Self::Kanabis => "\"".into(),
            Self::Jasenėc => "\"".into(),
        }
    }
}

impl BushType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(228, 46, 103),
        }
    }
}

impl MammalType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(210, 180, 140),
        }
    }
}

impl FishType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(102, 205, 170),
        }
    }
}

impl BirdType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(128, 128, 0),
        }
    }
}

impl LizardType {
    pub fn to_color(&self) -> Color {
        match &self {
            _ => Color::Rgb(0, 128, 128),
        }
    }
}

impl AnimalType {
    pub fn random_animaltype(small_rngik: &mut SmallRng) -> AnimalType {
        let y: f64 = small_rngik.gen(); // generates a float between 0 and 1

        if y < 0.5 {
            AnimalType::Mammal(MammalType::random_mammal_type(small_rngik))
        } else if y < 0.8 {
            AnimalType::Bird(BirdType::random_bird_type(small_rngik))
        } else {
            AnimalType::Lizard(LizardType::random_lizard_type(small_rngik))
        }
    }

    pub fn minimal_string(&self) -> String {
        match &self {
            AnimalType::Bird(x) => {
                format!("{}", &x)
            }
            AnimalType::Mammal(x) => {
                format!("{}", &x)
            }
            AnimalType::Lizard(x) => {
                format!("{}", &x)
            }
            AnimalType::Fish(x) => {
                format!("{}", &x)
            }
        }
    }

    pub fn to_displaychar(&self) -> String {
        let item_str = self.minimal_string();

        // let item_str = format!("{}", self.item_type);
        let ch = item_str.chars().nth(0).unwrap().to_string();
        ch
    }
    pub fn to_color(&self) -> Color {
        match &self {
            AnimalType::Bird(x) => x.to_color(),
            AnimalType::Mammal(x) => x.to_color(),
            AnimalType::Lizard(x) => x.to_color(),
            AnimalType::Fish(x) => x.to_color(),
        }
    }
}

impl MeleeWeapon {
    pub fn minimal_string(&self) -> String {
        format! {"{}",self.weapon_type}
    }
}
impl MeleeWeaponType {
    pub fn weapon_range(&self) -> StatsUnit {
        match self {
            MeleeWeaponType::Kopje => 10,
            //   MeleeWeaponType::Nož =>{100}
            _ => 3,
        }
    }
}

impl RangedWeapon {
    pub fn to_color(&self) -> Color {
        match &self.weapon_type {
            _ => self.tetiva_material.to_color(),
        }
    }

    pub fn minimal_string(&self) -> String {
        format! {"{}",self.weapon_type}
    }
}

impl ItemType {
    pub fn to_displaychar(&self) -> String {
        let item_str = self.minimal_string();

        // let item_str = format!("{}", self.item_type);
        let ch = item_str.chars().nth(0).unwrap().to_lowercase().to_string();
        ch
    }
    pub fn to_color(&self) -> Color {
        match &self {
            ItemType::Melee(x) => x.material_type.to_color(),
            ItemType::Ranged(x) => x.tetiva_material.to_color(),
            ItemType::Clothing(x) => x.fabric_type.to_color(),

            ItemType::None => Color::LightRed,
        }
    }
    pub fn minimal_string(&self) -> String {
        let item_str = match &self {
            ItemType::Melee(x) => {
                format!("{}", &x.weapon_type)
            }
            ItemType::Clothing(x) => {
                format!("{}", &x.clothing_type)
            }
            ItemType::Ranged(x) => {
                format!("{}", &x.weapon_type)
            }

            ItemType::None => "?".into(),
        };

        item_str
    }
}

impl Default for Human {
    fn default() -> Self {
        Self {
            inventory: Vec::new(),
            equipment: EquipmentComponent::new_hunter(),
            stats: StatsComponent::new_default(),
            cur_health: HealthComponent::new(),
            max_health: HealthComponent::new(),
            name: NameComponent::new(),
        }
    }
}

impl PlantType {
    pub fn to_displaychar(&self) -> String {
        match self {
            PlantType::Drěvo(x) => "t".into(),
            PlantType::Kust(x) => "*".into(),
            PlantType::Trava(x) => x.to_displaychar(),
        }
    }
    pub fn minimal_string(&self) -> String {
        match self {
            PlantType::Drěvo(x) => {
                format! {"{}",x}
            }
            PlantType::Kust(x) => {
                format! {"{}",x}
            }
            PlantType::Trava(x) => {
                format! {"{}",x}
            }
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            PlantType::Drěvo(x) => x.to_color(),
            PlantType::Kust(x) => x.to_color(),

            PlantType::Trava(x) => x.to_color(),
        }
    }
}

impl EntityType {
    pub fn minimal_string(&self) -> String {
        match self {
            EntityType::Human(x) => x.name.name.clone(),
            EntityType::Item(x) => x.minimal_string(),
            EntityType::Monster(x) => x.animal_type.minimal_string(),
            EntityType::Mebelj(x) => x.minimal_string(),
            EntityType::Råstlina(x) => x.minimal_string(),
            EntityType::None => String::new(),
        }
    }
    pub fn random_animal(small_rngik: &mut SmallRng) -> EntityType {
        EntityType::Monster(Animal {
            animal_type: AnimalType::random_animaltype(small_rngik),
            cur_health: HealthComponent::new(),
            max_health: HealthComponent::new(),
        })
    }
    pub fn to_displaychar(&self) -> String {
        match self {
            EntityType::Item(x) => x.to_displaychar(),
            EntityType::Monster(x) => x.animal_type.to_displaychar(), //x.to_displaychar(),
            EntityType::Human(_) => "𖣊".into(),
            EntityType::None => "?".into(),

            EntityType::Råstlina(x) => x.to_displaychar(),
            EntityType::Mebelj(x) => x.to_displaychar(),
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            EntityType::Item(x) => x.to_color(),
            EntityType::Monster(x) => x.animal_type.to_color(),
            EntityType::Human(_) => Color::White,
            EntityType::None => Color::Red,

            EntityType::Mebelj(x) => x.to_color(),
            EntityType::Råstlina(x) => x.to_color(),
        }
    }

    pub fn to_graphictriple(&self) -> GraphicTriple {
        let ent_char = self.to_displaychar();
        let ent_color = self.to_color();
        (ent_char, ent_color, Color::Black)
    }
}

impl Mebelj {
    pub fn minimal_string(&self) -> String {
        format!("{}", self.mebelj_type)
    }

    pub fn to_displaychar(&self) -> String {
        match &self.mebelj_type {
            MebeljType::Stěna => "#".into(),
            MebeljType::Dvėrj => "+".into(),
            MebeljType::Vråta => "=".into(),
            _ => todo!("implement mebelj"),
        }
    }
    pub fn to_color(&self) -> Color {
        self.material.to_color()
    }
}

impl MammalType {
    pub fn random_mammal_type(small_rngik: &mut SmallRng) -> MammalType {
        let y: f64 = small_rngik.gen(); // generates a float between 0 and 1
        let funny_index = y * MammalType::COUNT as f64;

        MammalType::from_repr(funny_index as usize).unwrap_or(MammalType::Jelenj)
    }
}
impl FishType {
    pub fn random_fish_type(small_rngik: &mut SmallRng) -> FishType {
        let y: f64 = small_rngik.gen(); // generates a float between 0 and 1
        let funny_index = y * FishType::COUNT as f64;

        FishType::from_repr(funny_index as usize).unwrap_or(FishType::Karas)
    }
}

impl BirdType {
    pub fn random_bird_type(small_rngik: &mut SmallRng) -> BirdType {
        let y: f64 = small_rngik.gen(); // generates a float between 0 and 1
        let funny_index = y * BirdType::COUNT as f64;

        BirdType::from_repr(funny_index as usize).unwrap_or(BirdType::Sova)
    }
}

impl LizardType {
    pub fn random_lizard_type(small_rngik: &mut SmallRng) -> LizardType {
        let y: f64 = small_rngik.gen(); // generates a float between 0 and 1
        let funny_index = y * LizardType::COUNT as f64;

        LizardType::from_repr(funny_index as usize).unwrap_or(LizardType::Gad)
    }
}
