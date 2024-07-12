use crate::*;

impl EntityType {
    pub fn minimal_string(&self) -> String {
        match self {
            EntityType::Human => "person".into(),
        }
    }

    pub fn to_displaychar(&self) -> String {
        match self {
            EntityType::Human => "@".into(),
        }
    }

    pub fn to_color(&self) -> RatColor {
        match self {
            EntityType::Human => RatColor::White,
        }
    }

    pub fn to_graphictriple(&self) -> GraphicTriple {
        let ent_char = self.to_displaychar();
        let ent_color = self.to_color();
        (ent_char, ent_color, RatColor::Black)
    }
}
