use crate::*;







impl Default for Human {
    fn default() -> Self {
        Self {
        
       
            stats: StatsComponent::new_default(),
            cur_health: HealthComponent::new(),
            max_health: HealthComponent::new(),
            name: NameComponent::new(),
        }
    }
}


impl EntityType {
    pub fn minimal_string(&self) -> String {
        match self {
            EntityType::Human(x) => x.name.name.clone(),
         
           
            
        }
    }
  
    pub fn to_displaychar(&self) -> String {
        match self {
           
          
            EntityType::Human(_) => "𖣊".into(),
       
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
         
           
            EntityType::Human(_) => Color::White,
   
        }
    }

    pub fn to_graphictriple(&self) -> GraphicTriple {
        let ent_char = self.to_displaychar();
        let ent_color = self.to_color();
        (ent_char, ent_color, Color::Black)
    }
}

