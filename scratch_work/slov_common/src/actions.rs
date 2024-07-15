use crate::*;

pub fn action_processor(
    mut point_action: Query<(Entity, &EntityType, &mut PointComponent, &ActionComponent)>,
    //   just_points: Query<(&PointComponent)>,
    mut masterok: ResMut<Masterok>,
) {
    for (eid, ent_type, mut e_pos, e_action) in point_action.iter_mut() {
        match e_action {
            ActionComponent::Go(direction) => {
                let dir = direction.to_xyz();
                let potential_loc = (e_pos.0 .0 + dir.0, e_pos.0 .1 + dir.1);
                attempt_move_ent_to_point(
                    &mut e_pos,
                    &potential_loc,
                    &mut masterok.game_map,
                    &eid,
                    &ent_type,
                )
            }
            _ => todo!("ACTION NOT IMPLEMENTED"),
        }
    }
}

pub fn attempt_move_ent_to_point(
    origin: &mut PointComponent,
    destination: &MyPoint,
    game_map: &mut GameMap,
    eid: &Entity,
    ent_type: &EntityType,
) {
    let mut dest_vox = game_map.get_mut_voxel_at(destination);

    let vox_blocks_movement = match dest_vox {
        Some(voxik) => voxik.blocks_movement(),
        None => true,
    };

    if !vox_blocks_movement {
        let mut dest_vox = game_map.get_mut_voxel_at(destination).unwrap();

        dest_vox.entity_map.insert(eid.clone(), ent_type.clone());

        let mut origin_vox = game_map.get_mut_voxel_at(&origin.0).unwrap();

        origin_vox.entity_map.remove(eid);

        origin.0 = destination.clone();
    }
}

pub fn action_remover(ent_action: Query<(Entity, &ActionComponent)>, mut commands: Commands) {
    for (eid, _) in ent_action.iter() {
        commands.entity(eid).remove::<ActionComponent>();
    }
}
