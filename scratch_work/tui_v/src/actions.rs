use crate::*;

impl App {
    pub fn handle_wait(&self, subject_eid: &EntityID) -> ActionResult {
        ActionResult::Success(GameAction::Wait(subject_eid.clone()))
    }
    pub fn handle_movement(
        &mut self,
        subject_eid: &EntityID,
        cd: &CardinalDirection,
    ) -> ActionResult {
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
    pub fn drop_item_from_inv(&mut self, subject_eid: &EntityID, item: &EntityID) -> ActionResult {
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

    pub fn equip_item_from_inv(&mut self, subject_eid: &EntityID, item: &EntityID) -> ActionResult {
        if let Some(boop) = self.components.equipments.get_mut(subject_eid) {
            if boop.inventory.contains(item) {
                let item_type = self.components.ent_types.get(item).unwrap();

                if let EntityType::Item(itemik) = item_type {
                    match itemik {
                        ItemType::Weapon(wep) => {}
                        ItemType::Clothing(cloth) => {}
                        ItemType::RangedWeapon(rang) => {}
                        ItemType::Ammo(amm) => {
                            for thing_equipped in &boop.equipped {
                                let thing_equipped_type =
                                    self.components.ent_types.get(thing_equipped).unwrap();
                                // if let E
                            }
                        }
                    }
                }

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

    pub fn pronoun_for_act_string(&self, subj: &EntityID) -> (String, Gender, Person) {
        let person = if subj == &self.local_player_id {
            Person::Second
        } else {
            Person::Third
        };

        let pn = match person {
            Person::Second => "ty".to_string(),
            Person::Third => self.get_entity_name(&subj),
            Person::First => "ja".to_string(),
        };

        if let Some(gender) = self.components.genders.get(subj) {
            (pn, gender.clone(), person)
        } else {
            let genik = ISV::guess_gender(&pn);
            (pn, genik, person)
        }
    }

    pub fn generate_action_result_string(&self, act_resut: ActionResult) -> Line {
        let line_text = match act_resut {
            ActionResult::Success(ga) => match ga {
                GameAction::Drop(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_string(&subj);
                    let object = ISV::decline_noun(
                        &self.get_entity_name(&obj),
                        &Case::Acc,
                        &Number::Singular,
                    );
                    let verbik = ISV::l_participle("opustiti", &gender, &Number::Singular);

                    format!("{pronoun} {verbik} {}", object.0)
                }
                GameAction::Equip(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_string(&subj);
                    let object = ISV::decline_noun(
                        &self.get_entity_name(&obj),
                        &Case::Acc,
                        &Number::Singular,
                    );
                    let verbik = ISV::conjugate_verb(
                        "equipirovati",
                        &person,
                        &Number::Singular,
                        &gender,
                        &Tense::Present,
                    );

                    format!("{pronoun} {verbik} {}", object.0)
                }
                GameAction::UnEquip(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_string(&subj);
                    let object = ISV::decline_noun(
                        &self.get_entity_name(&obj),
                        &Case::Acc,
                        &Number::Singular,
                    );
                    let verbik = ISV::l_participle("opustiti", &gender, &Number::Singular);

                    format!("{pronoun} {verbik} {}", object.0)
                }
                GameAction::Go(subj, cd) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_string(&subj);
                    let dropped = cd.to_isv();

                    format!("{pronoun} poszol na {dropped}")
                }
                GameAction::PickUp(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_string(&subj);
                    let object = ISV::decline_noun(
                        &self.get_entity_name(&obj),
                        &Case::Acc,
                        &Number::Singular,
                    );
                    let verbik = ISV::l_participle("opustiti", &gender, &Number::Singular);

                    format!("{pronoun} {verbik} {}", object.0)
                }
                GameAction::Wait(subj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_string(&subj);

                    let verbik = ISV::l_participle("počekati", &gender, &Number::Singular);

                    format!("{pronoun} {verbik}")
                }
                _ => panic!("NOT IMPLEMENTED"),
            },
            ActionResult::Failure(ga) => match ga {
                GameAction::Drop(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_string(&subj);
                    let dropped = self.get_entity_name(&obj);

                    format!("{pronoun} brosil {dropped}")
                }
                GameAction::Equip(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_string(&subj);
                    let dropped = self.get_entity_name(&obj);

                    format!("{pronoun} brosil {dropped}")
                }
                GameAction::UnEquip(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_string(&subj);
                    let dropped = self.get_entity_name(&obj);

                    format!("{pronoun} brosil {dropped}")
                }
                GameAction::Go(subj, cd) => "ne mozzesz tuda idti".to_string(),
                GameAction::PickUp(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_string(&subj);
                    let dropped = self.get_entity_name(&obj);

                    format!("{pronoun} brosil {dropped}")
                }
                _ => panic!("NOT IMPLEMENTED"),
            },
        };

        Line::from(line_text)
    }
}
