use crate::*;

#[derive(Component)]
pub struct Player;

type XPosition = i64;
type YPosition = i64;
//type ZPosition = i64;

#[derive(Component)]
pub struct GamePosition {
    pub x: XPosition,
    pub y: YPosition,
    //  z: ZPosition
}

impl GamePosition {
    pub fn new() -> GamePosition {
        GamePosition { x: 5, y: 5 }
    }
}

#[derive(Component)]
pub struct GameRenderable {
    pub display_char: String,
    pub fg_color: RatColor,
    pub bg_color: RatColor,
}

impl GameRenderable {
    pub fn new_human() -> GameRenderable {
        GameRenderable {
            display_char: "@".into(),
            fg_color: RatColor::White,
            bg_color: RatColor::Black,
        }
    }

    pub fn to_graphictriple(&self) -> GraphicTriple {
        (
            self.display_char.clone(),
            self.fg_color.clone(),
            RatColor::Black,
        )
    }
}
