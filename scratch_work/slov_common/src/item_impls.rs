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
   
        }
    }
    pub fn minimal_string(&self) -> String {
        let item_str = match &self {
            ItemType::Melee(x) => {
                format!("{}", &x.weapon_type)
            }
  
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
           
            EntityType::Mebelj(x) => x.minimal_string(),
            EntityType::Råstlina(x) => x.minimal_string(),
            EntityType::None => String::new(),
        }
    }
  
    pub fn to_displaychar(&self) -> String {
        match self {
            EntityType::Item(x) => x.to_displaychar(),
          
            EntityType::Human(_) => "𖣊".into(),
            EntityType::None => "?".into(),

            EntityType::Råstlina(x) => x.to_displaychar(),
            EntityType::Mebelj(x) => x.to_displaychar(),
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            EntityType::Item(x) => x.to_color(),
           
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

