use crate::*;

pub fn action_processor(
    mut point_action: Query<(&mut PointComponent, &ActionComponent)>,
 //   just_points: Query<(&PointComponent)>,
    masterok: Res<Masterok>,
) {
    for (mut e_pos, e_action) in point_action.iter_mut() {
        match e_action {
            ActionComponent::Go(direction) => { let potential_loc = match direction {
                CardinalDirection::North => {
                   e_pos.0 .1 + 1
                }
                CardinalDirection::South => {
                    e_pos.0 .1 - 1
                }
                CardinalDirection::West => {
                    e_pos.0 .0 -1
                }
                CardinalDirection::East => {
                    e_pos.0 .0 + 1
                }
            };},
            _ => todo!("ACTION NOT IMPLEMENTED"),
        }
    }
}

pub fn action_remover(ent_action: Query<(Entity, &ActionComponent)>, mut commands: Commands) {
    for (eid, _) in ent_action.iter() {
        commands.entity(eid).remove::<ActionComponent>();
    }
}
