use crate::*;

impl App {




    pub fn handle_movement(&mut self, subject_eid: &EntityID, cd: &CardinalDirection) -> ActionResult {
        let xyik = cd.to_xyz();
        if let Some(e_pos) = self.components.positions.get_mut(subject_eid) {
            let destination = (e_pos.0 + xyik.0, e_pos.1 + xyik.1);
            // println!("epos got");

            if let Some(dest_vox) = self.game_map.get_mut_voxel_at(&destination) {
                if !dest_vox.blocks_movement(&self.components.ent_types) {
                    // println!("dest no block");
                    dest_vox.entity_set.insert(subject_eid.clone());

                    if let Some(origin_vox) = self.game_map.get_mut_voxel_at(e_pos) {
                        origin_vox.entity_set.remove(subject_eid);
                    }
                    *e_pos = destination;
                    return ActionResult::Success(GameAction::Go(subject_eid.clone(), cd.clone()));
                }
            }
        }

        ActionResult::Failure(GameAction::Go(subject_eid.clone(), cd.clone()))
    }
    pub fn drop_item_from_inv(&mut self,    subject_eid: &EntityID,
        item: &EntityID,) -> ActionResult {
        let subject_eid_inv = self.components.equipments.get_mut(subject_eid).unwrap();
        let subject_eid_pos = self.components.positions.get(subject_eid).unwrap();

        if subject_eid_inv.inventory.contains(item) {
            subject_eid_inv.inventory.remove(item);
            let subject_eid_vox = self.game_map.get_mut_voxel_at(subject_eid_pos).unwrap();
            subject_eid_vox.entity_set.insert(item.clone());
            return ActionResult::Success(GameAction::Drop(subject_eid.clone(), item.clone()));
        }
        ActionResult::Failure(GameAction::Drop(subject_eid.clone(), item.clone()))
    }

    pub fn pickup_item_from_ground(
        &mut self,
        subject_eid: &EntityID,
        item: &EntityID,
    ) -> ActionResult {
        if let Some(ent_pos) = self.components.positions.get(subject_eid) {
            if let Some(ent_vox) = self.game_map.get_mut_voxel_at(ent_pos) {
                if ent_vox.entity_set.contains(item) {
                    ent_vox.entity_set.remove(item);
                    self.components
                        .equipments
                        .get_mut(subject_eid)
                        .unwrap()
                        .inventory
                        .insert(item.clone());
                    return ActionResult::Success(GameAction::PickUp(
                        subject_eid.clone(),
                        item.clone(),
                    ));
                }
            }
        }
        ActionResult::Failure(GameAction::PickUp(subject_eid.clone(), item.clone()))
    }

    pub fn equip_item_from_inv(&mut self,    subject_eid: &EntityID,
        item: &EntityID,) -> ActionResult {
        if let Some(boop) = self.components.equipments.get_mut(subject_eid) {
            if boop.inventory.contains(item) {
                boop.inventory.remove(item);
                boop.equipped.insert(item.clone());
                return ActionResult::Success(GameAction::Equip(subject_eid.clone(), item.clone()));
            }
        }
        return ActionResult::Failure(GameAction::Equip(subject_eid.clone(), item.clone()));
    }
    pub fn unequip_item_from_equipped(
        &mut self,
        subject_eid: &EntityID,
        item: &EntityID,
        
    ) -> ActionResult {
        if let Some(boop) = self.components.equipments.get_mut(subject_eid) {
            if boop.equipped.contains(item) {
                boop.equipped.remove(item);
                boop.inventory.insert(item.clone());
                return ActionResult::Success(GameAction::UnEquip(
                    subject_eid.clone(),
                    item.clone(),
                ));
            }
        }
        return ActionResult::Failure(GameAction::UnEquip(subject_eid.clone(), item.clone()));
    }
}
