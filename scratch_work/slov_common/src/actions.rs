use crate::*;



pub fn action_processor(mut player_position: Query<(&mut PointComponent, &ActionComponent)>) {
    for (mut e_pos, e_action) in player_position.iter_mut() {
        match e_action {
            ActionComponent::Go(direction) => match direction {
                CardinalDirection::North => {
                    e_pos.0.1 += 1;
                }
                CardinalDirection::South => {
                    e_pos.0.1 -= 1;
                }
                CardinalDirection::West => {
                    e_pos.0.0 -= 1;
                }
                CardinalDirection::East => {
                    e_pos.0.0 += 1;
                }
            },
            _ => todo!("ACTION NOT IMPLEMENTED"),
        }
    }
}

pub fn action_remover(player_position: Query<(Entity, &ActionComponent)>, mut commands: Commands) {
    for (eid, _) in player_position.iter() {
        commands.entity(eid).remove::<ActionComponent>();
    }
}
