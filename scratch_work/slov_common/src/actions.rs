use crate::*;

pub fn action_processor(
    mut point_action: Query<(&mut PointComponent, &ActionComponent)>,
 //   just_points: Query<(&PointComponent)>,
    masterok: Res<Masterok>,
) {
    for (mut e_pos, e_action) in point_action.iter_mut() {
        match e_action {
            ActionComponent::Go(direction) => { 
                let dir = direction.to_xyz();
                let potential_loc = (e_pos.0.0 + dir.0 , e_pos.0.1 + dir.1);

            let vox = masterok.game_map.get_voxel_at(&potential_loc);

            let vox_blocks_movement =  match vox {
                Some(voxik) => voxik.blocks_movement(),
                None => true
            };

            if !vox_blocks_movement {e_pos.0 = potential_loc;}
        
        
        

        
        
        },
            _ => todo!("ACTION NOT IMPLEMENTED"),
        }
    }
}

pub fn action_remover(ent_action: Query<(Entity, &ActionComponent)>, mut commands: Commands) {
    for (eid, _) in ent_action.iter() {
        commands.entity(eid).remove::<ActionComponent>();
    }
}
