use crate::*;

// Define a marker trait
#[derive(Display, Clone)]
pub enum ActionResult {
    Success(GameAction, SuccessType),
    Failure(GameAction, FailType),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SuccessType {
    Normal,
    WithValue(i64), //for damage
    WithValueAndRangedWeapon(i64, RangedWeapon),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FailType {
    Normal,
    Miss,
    MissWithInstrument(EntityID),
    NoAmmo,
    NoWeapon,
    Blocked,
    InventoryFull,
    WrongType,
    AlreadyEquipped,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GameAction {
    Wait(Subject),
    PickUp(Subject, DirectObject),
    Equip(Subject, DirectObject),
    UnEquip(Subject, DirectObject),
    BumpAttack(Subject, DirectObject),
    RangedAttack(Subject, DirectObject),

    Drop(Subject, DirectObject),

    Go(Subject, CardinalDirection),
}

impl App {
    pub fn handle_wait(&self, subject_eid: &EntityID) -> ActionResult {
        ActionResult::Success(GameAction::Wait(subject_eid.clone()), SuccessType::Normal)
    }

    pub fn get_ent_melee_damage(&self, subject_eid: &EntityID) -> i64 {
        let mut base_damage = 0;
        if let Some(equi) = self.components.equipments.get(subject_eid) {
            for itemik in &equi.equipped {
                if let EntityType::Item(typik) = self.get_ent_type(itemik) {
                    match typik {
                        ItemType::Weapon(wepik) => base_damage = base_damage + wepik.damage(),
                        _ => {}
                    }
                }
            }
        }
        base_damage
    }

    pub fn ranged_attack(&mut self, subject_eid: &EntityID, object_eid: &EntityID) -> ActionResult {
        let mut base_damage = 0;
        if let Some(equi) = self.components.equipments.get(subject_eid) {
            for itemik in &equi.equipped {
                if let EntityType::Item(typik) = self.get_ent_type(itemik) {
                    match typik {
                        ItemType::RangedWeapon(wepik) => {
                            base_damage = base_damage + wepik.damage();
                            if let Some(defender_health) =
                                self.components.healths.get_mut(object_eid)
                            {
                                defender_health.current_health -= base_damage;
                                return ActionResult::Success(
                                    GameAction::RangedAttack(
                                        subject_eid.clone(),
                                        object_eid.clone(),
                                    ),
                                    SuccessType::WithValueAndRangedWeapon(
                                        base_damage,
                                        wepik.clone(),
                                    ),
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
            return ActionResult::Failure(
                GameAction::RangedAttack(subject_eid.clone(), object_eid.clone()),
                FailType::NoWeapon,
            );
        }

        return ActionResult::Failure(
            GameAction::RangedAttack(subject_eid.clone(), object_eid.clone()),
            FailType::NoWeapon,
        );
    }

    pub fn bump_attack(&mut self, subject_eid: &EntityID, object_eid: &EntityID) -> ActionResult {
        let attacker_damage = self.get_ent_melee_damage(subject_eid);
        if let Some(defender_health) = self.components.healths.get_mut(object_eid) {
            defender_health.current_health -= attacker_damage;
            return ActionResult::Success(
                GameAction::BumpAttack(subject_eid.clone(), object_eid.clone()),
                SuccessType::WithValue(attacker_damage),
            );
        }

        ActionResult::Failure(
            GameAction::BumpAttack(subject_eid.clone(), object_eid.clone()),
            FailType::Normal,
        )
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
                //check for bump attack
                for dest_ent in &dest_vox.entity_set {
                    let typik = self.components.ent_types.get(dest_ent).unwrap();

                    if typik.is_attackable() {
                        let wut = dest_ent.clone();
                        let resultatik = self.bump_attack(subject_eid, &wut);
                        return resultatik;
                    }
                }

                //no bump attack, then check if something blocks movement
                if !dest_vox.blocks_movement(&self.components.ent_types) {
                    // println!("dest no block");
                    dest_vox.entity_set.push(subject_eid.clone());

                    if let Some(origin_vox) = self.game_map.get_mut_voxel_at(e_pos) {
                        remove_ent_from_vec(&mut origin_vox.entity_set, subject_eid);
                    }
                    *e_pos = destination;
                    return ActionResult::Success(
                        GameAction::Go(subject_eid.clone(), cd.clone()),
                        SuccessType::Normal,
                    );
                }
            }
        }

        ActionResult::Failure(
            GameAction::Go(subject_eid.clone(), cd.clone()),
            FailType::Blocked,
        )
    }
    pub fn drop_item_from_inv(&mut self, subject_eid: &EntityID, item: &EntityID) -> ActionResult {
        let subject_eid_inv = self.components.equipments.get_mut(subject_eid).unwrap();
        let subject_eid_pos = self.components.positions.get(subject_eid).unwrap();

        if subject_eid_inv.inventory.contains(item) {
            subject_eid_inv.inventory.remove(item);
            let subject_eid_vox = self.game_map.get_mut_voxel_at(subject_eid_pos).unwrap();
            subject_eid_vox.entity_set.push(item.clone());
            return ActionResult::Success(
                GameAction::Drop(subject_eid.clone(), item.clone()),
                SuccessType::Normal,
            );
        }
        ActionResult::Failure(
            GameAction::Drop(subject_eid.clone(), item.clone()),
            FailType::Normal,
        )
    }

    pub fn pickup_item_from_ground(
        &mut self,
        subject_eid: &EntityID,
        item: &EntityID,
    ) -> ActionResult {
        if let Some(ent_pos) = self.components.positions.get(subject_eid) {
            if let Some(ent_vox) = self.game_map.get_mut_voxel_at(ent_pos) {
                if ent_vox.entity_set.contains(item) {
                    remove_ent_from_vec(&mut ent_vox.entity_set, item);

                    self.components
                        .equipments
                        .get_mut(subject_eid)
                        .unwrap()
                        .inventory
                        .insert(item.clone());
                    return ActionResult::Success(
                        GameAction::PickUp(subject_eid.clone(), item.clone()),
                        SuccessType::Normal,
                    );
                }
            }
        }
        ActionResult::Failure(
            GameAction::PickUp(subject_eid.clone(), item.clone()),
            FailType::Normal,
        )
    }

    pub fn equip_item_from_inv(&mut self, subject_eid: &EntityID, item: &EntityID) -> ActionResult {
        if let Some(boop) = self.components.equipments.get_mut(subject_eid) {
            if boop.inventory.contains(item) {
                let item_type = self.components.ent_types.get(item).unwrap();

                if let EntityType::Item(itemik) = item_type {
                    match itemik {
                        ItemType::Weapon(wep) => {
                            let mut hand_space: i64 = 2;
                            for thing_equipped in &boop.equipped {
                                if let EntityType::Item(ItemType::Weapon(wepik)) =
                                    self.components.ent_types.get(thing_equipped).unwrap()
                                {
                                    hand_space = hand_space - wepik.handedness();
                                }
                            }
                            if wep.handedness() <= hand_space {
                                boop.inventory.remove(item);
                                boop.equipped.insert(item.clone());
                                return ActionResult::Success(
                                    GameAction::Equip(subject_eid.clone(), item.clone()),
                                    SuccessType::Normal,
                                );
                            } else {
                                return ActionResult::Failure(
                                    GameAction::Equip(subject_eid.clone(), item.clone()),
                                    FailType::AlreadyEquipped,
                                );
                            }
                        }
                        ItemType::Accessory(acc) => {
                            let mut accessory_space: i64 = 4;
                            for thing_equipped in &boop.equipped {
                                if let EntityType::Item(ItemType::Accessory(accik)) =
                                    self.components.ent_types.get(thing_equipped).unwrap()
                                {
                                    accessory_space = accessory_space - 1;
                                }
                            }
                            if 0 < accessory_space {
                                boop.inventory.remove(item);
                                boop.equipped.insert(item.clone());
                                return ActionResult::Success(
                                    GameAction::Equip(subject_eid.clone(), item.clone()),
                                    SuccessType::Normal,
                                );
                            } else {
                                return ActionResult::Failure(
                                    GameAction::Equip(subject_eid.clone(), item.clone()),
                                    FailType::AlreadyEquipped,
                                );
                            }
                        }
                        ItemType::Clothing(cloth) => {
                            let new_part_covered = cloth.body_part_covered();
                            for thing_equipped in &boop.equipped {
                                if let Some(EntityType::Item(ItemType::Clothing(clothik))) =
                                    self.components.ent_types.get(thing_equipped)
                                {
                                    let exist_part_covered = clothik.body_part_covered();
                                    if new_part_covered == exist_part_covered {
                                        return ActionResult::Failure(
                                            GameAction::Equip(subject_eid.clone(), item.clone()),
                                            FailType::AlreadyEquipped,
                                        );
                                    }
                                }
                            }
                            boop.inventory.remove(item);
                            boop.equipped.insert(item.clone());
                            return ActionResult::Success(
                                GameAction::Equip(subject_eid.clone(), item.clone()),
                                SuccessType::Normal,
                            );
                        }
                        ItemType::RangedWeapon(rang) => {
                            for thing_equipped in &boop.equipped {
                                if let Some(EntityType::Item(ItemType::RangedWeapon(ex_rang))) =
                                    self.components.ent_types.get(thing_equipped)
                                {
                                    return ActionResult::Failure(
                                        GameAction::Equip(subject_eid.clone(), item.clone()),
                                        FailType::AlreadyEquipped,
                                    );
                                }
                            }
                            boop.inventory.remove(item);
                            boop.equipped.insert(item.clone());
                            return ActionResult::Success(
                                GameAction::Equip(subject_eid.clone(), item.clone()),
                                SuccessType::Normal,
                            );
                        }
                        ItemType::Ammo(amm) => {
                            //cant equip ammo it is used directly from inventory
                            return ActionResult::Failure(
                                GameAction::Equip(subject_eid.clone(), item.clone()),
                                FailType::WrongType,
                            );
                        }
                    }
                }
            }
        }
        return ActionResult::Failure(
            GameAction::Equip(subject_eid.clone(), item.clone()),
            FailType::Normal,
        );
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
                return ActionResult::Success(
                    GameAction::UnEquip(subject_eid.clone(), item.clone()),
                    SuccessType::Normal,
                );
            }
        }
        return ActionResult::Failure(
            GameAction::UnEquip(subject_eid.clone(), item.clone()),
            FailType::Normal,
        );
    }

    pub fn pronoun_for_act_subj(&self, subj: &EntityID) -> (String, Gender, Person) {
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
    pub fn pronoun_for_act_obj(&self, subj: &EntityID) -> (String, Gender, Person) {
        let person = if subj == &self.local_player_id {
            Person::Second
        } else {
            Person::Third
        };

        let pn = match person {
            Person::Second => "tę".to_string(),
            Person::Third => {
                ISV::decline_noun(&self.get_entity_name(&subj), &Case::Acc, &Number::Singular).0
            }
            Person::First => "ja".to_string(),
        };

        if let Some(gender) = self.components.genders.get(subj) {
            (pn, gender.clone(), person)
        } else {
            let genik = ISV::guess_gender(&self.get_entity_name(&subj));
            (pn, genik, person)
        }
    }

    pub fn generate_action_result_string(&self, act_resut: ActionResult) -> Line {
        let line_text = match act_resut {
            ActionResult::Success(ga, reason) => match ga {
                GameAction::Drop(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_subj(&subj);
                    let object = ISV::decline_noun(
                        &self.get_entity_name(&obj),
                        &Case::Acc,
                        &Number::Singular,
                    );
                    let verbik = ISV::l_participle("opustiti", &gender, &Number::Singular);

                    format!("{pronoun} {verbik} {}", object.0)
                }
                GameAction::Equip(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_subj(&subj);

                    let objtyp = self.get_ent_type(&obj);

                    let equip_verb = match objtyp {
                        EntityType::Item(itemik) => match itemik {
                            (ItemType::Weapon(_) | ItemType::RangedWeapon(_)) => "orųžati",
                            ItemType::Clothing(_) => "oděvati",
                            _ => "equipirovati",
                        },
                        _ => "equipirovati",
                    };
                    let verbik = ISV::conjugate_verb(
                        equip_verb,
                        &person,
                        &Number::Singular,
                        &gender,
                        &Tense::Present,
                    );

                    if equip_verb == "orųžati" {
                        let object = ISV::decline_noun(
                            &self.get_entity_name(&obj),
                            &Case::Ins,
                            &Number::Singular,
                        );
                        format!("{pronoun} {verbik} sę {}", object.0)
                    } else {
                        let object = ISV::decline_noun(
                            &self.get_entity_name(&obj),
                            &Case::Acc,
                            &Number::Singular,
                        );
                        format!("{pronoun} {verbik} {}", object.0)
                    }
                }
                GameAction::UnEquip(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_subj(&subj);
                    let object = ISV::decline_noun(
                        &self.get_entity_name(&obj),
                        &Case::Acc,
                        &Number::Singular,
                    );
                    let verbik = ISV::conjugate_verb(
                        "odkladati",
                        &person,
                        &Number::Singular,
                        &gender,
                        &Tense::Present,
                    );

                    format!("{pronoun} {verbik} {}", object.0)
                }
                GameAction::Go(subj, cd) => {
                    format!("")
                }
                GameAction::PickUp(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_subj(&subj);
                    let object = ISV::decline_noun(
                        &self.get_entity_name(&obj),
                        &Case::Acc,
                        &Number::Singular,
                    );
                    let verbik = ISV::conjugate_verb(
                        "podbirati",
                        &person,
                        &Number::Singular,
                        &gender,
                        &Tense::Present,
                    );

                    format!("{pronoun} {verbik} {}", object.0)
                }
                GameAction::Wait(subj) => {
                    if &subj == &self.local_player_id {
                        let (pronoun, gender, person) = self.pronoun_for_act_subj(&subj);

                        let verbik = ISV::l_participle("počekati", &gender, &Number::Singular);

                        format!("{pronoun} {verbik}")
                    } else {
                        format!("")
                    }
                }
                GameAction::BumpAttack(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_subj(&subj);
                    let extra_string = match reason {
                        SuccessType::WithValue(val) => {
                            let verbik = ISV::conjugate_verb(
                                "nanositi",
                                &person,
                                &Number::Singular,
                                &gender,
                                &Tense::Present,
                            );
                            format!(", {verbik} {val} točky škody")
                        }
                        _ => format!(""),
                    };
                    let object = self.pronoun_for_act_obj(&obj);

                    let verbik = ISV::conjugate_verb(
                        "atakovati",
                        &person,
                        &Number::Singular,
                        &gender,
                        &Tense::Present,
                    );

                    format!("{pronoun} {verbik} {}{extra_string}", object.0)
                }
                GameAction::RangedAttack(subj, obj) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_subj(&subj);
                    let object = self.pronoun_for_act_obj(&obj);

                    match reason {
                        SuccessType::WithValueAndRangedWeapon(val, inst) => {
                            let nanositi = ISV::conjugate_verb(
                                "nanositi",
                                &person,
                                &Number::Singular,
                                &gender,
                                &Tense::Present,
                            );
                            let attack_verb = match inst {
                                RangedWeapon::Lųk => "strelati",
                                RangedWeapon::Pråšča => "metati",
                            };

                            let instrument = ISV::decline_noun(
                                &format!("{}", inst),
                                &Case::Ins,
                                &Number::Singular,
                            )
                            .0;
                            let verbik = ISV::conjugate_verb(
                                attack_verb,
                                &person,
                                &Number::Singular,
                                &gender,
                                &Tense::Present,
                            );
                            format!(
                                "{pronoun} {verbik} {instrument} v {} , {nanositi} {val} točky škody",
                                object.0
                            )
                        }
                        _ => {
                            let verbik = ISV::conjugate_verb(
                                "atakovati",
                                &person,
                                &Number::Singular,
                                &gender,
                                &Tense::Present,
                            );

                            format!("{pronoun} {verbik} {}", object.0)
                        }
                    }
                }
            },
            ActionResult::Failure(ga, reason) => match ga {
                GameAction::Drop(subj, obj) => {
                    if &subj == &self.local_player_id {
                        let dropped = self.pronoun_for_act_obj(&obj).0;

                        format!("ty ne možeš opustiti {dropped}")
                    } else {
                        format!("")
                    }
                }
                GameAction::Equip(subj, obj) => {
                    let extra_string = match reason {
                        FailType::AlreadyEquipped => ",podobna věć uže orųdujųća ",
                        _ => "",
                    };
                    if &subj == &self.local_player_id {
                        let obje = self.pronoun_for_act_obj(&obj).0;

                        format!("ty ne možeš equipirovati {obje}{extra_string}")
                    } else {
                        format!("")
                    }
                }
                GameAction::UnEquip(subj, obj) => {
                    if &subj == &self.local_player_id {
                        let dropped = self.pronoun_for_act_obj(&obj).0;

                        format!("ty ne možeš odklasti {dropped}")
                    } else {
                        format!("")
                    }
                }
                GameAction::Go(subj, cd) => {
                    if &subj == &self.local_player_id {
                        format!("ty ne možeš tųdy idti")
                    } else {
                        format!("")
                    }
                }
                GameAction::PickUp(subj, obj) => {
                    if &subj == &self.local_player_id {
                        let dropped = self.pronoun_for_act_obj(&obj).0;

                        format!("ty ne možeš vzęti {dropped}")
                    } else {
                        format!("")
                    }
                }
                GameAction::RangedAttack(subj, obj) => {
                    if &subj == &self.local_player_id {
                        let dropped = self.pronoun_for_act_obj(&obj).0;

                        format!("ty ne možeš atakovati {dropped}")
                    } else {
                        format!("")
                    }
                }
                GameAction::BumpAttack(subj, obj) => {
                    if &subj == &self.local_player_id {
                        let dropped = self.pronoun_for_act_obj(&obj).0;

                        format!("ty ne možeš atakovati {dropped}")
                    } else {
                        format!("")
                    }
                }
                GameAction::Wait(subj) => {
                    format!("")
                }
            },
        };

        Line::from(line_text)
    }
}
