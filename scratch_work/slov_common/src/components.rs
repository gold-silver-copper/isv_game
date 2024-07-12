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


// for horse vehicle use horse chess piece as head  ∧ ♞

// projectile weapons have two parts (path to take, and steps of path to take, something flying slow and not far will have low each, something flying slow but far will ahve lots of steps, this can be passed to Go()
//MAKE IT POSSIBLE TO SET PLAYER AI MODE FOR AUTO PLAYING
#[derive(Clone, Debug, PartialEq, Component)]
pub enum ActionComponent {
    Wait,
    Take(),
    MeleeAttack(),
    Drop(),
    Give(),
    Hit(),
    Go(CardinalDirection),
    Quit,
}