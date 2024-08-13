use crate::*;


pub struct Player;

pub type MyPoint = (CoordinateUnit, CoordinateUnit);
pub type GraphicTriple = (&'static str, RatColor, RatColor);

pub struct PointComponent(pub MyPoint);


pub struct GraphicComponent(pub GraphicTriple);

// for horse vehicle use horse chess piece as head  ∧ ♞

// projectile weapons have two parts (path to take, and steps of path to take, something flying slow and not far will have low each, something flying slow but far will ahve lots of steps, this can be passed to Go()
//MAKE IT POSSIBLE TO SET PLAYER AI MODE FOR AUTO PLAYING
#[derive(Clone, Debug, PartialEq)]
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
