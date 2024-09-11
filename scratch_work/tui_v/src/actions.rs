use rand::Rng;

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
    WithValue(i32), //for damage

    WithValueAndRangedWeapon(i32, RangedWeapon),
}
#[derive(Clone, Debug, PartialEq)]
pub enum FailType {
    Normal,
    Miss,
    MissWithInstrument(EntityID),
    NoAmmo,

    Blocked,
    InventoryFull,
    WrongType,
    AlreadyEquipped,
}

#[derive(Clone, PartialEq)]
pub enum GameAction {
    Wait(Subject),
    PickUp(Subject, DirectObject),
    Equip(Subject, DirectObject),
    UnEquip(Subject, DirectObject),
    BumpAttack(Subject, DirectObject),
    RangedAttack(Subject, DirectObject),
    Death(Subject),
    Consume(Subject, Consumable),

    Drop(Subject, DirectObject),

    Go(Subject, CardinalDirection),
}

impl App {
    pub fn handle_wait(&self, subject_eid: &EntityID) -> ActionResult {
        ActionResult::Success(GameAction::Wait(subject_eid.clone()), SuccessType::Normal)
    }
    pub fn level_up_strength(&mut self, subject_eid: &EntityID) {
        if let Some(stats) = self.components.stats.get_mut(subject_eid) {
            let chance = self.small_rng.gen_ratio(1, (stats.strength * 2) as u32);
            if chance {
                stats.strength += 1;
            }
        }
    }
    pub fn level_up_speed(&mut self, subject_eid: &EntityID) {
        if let Some(stats) = self.components.stats.get_mut(subject_eid) {
            let chance = self.small_rng.gen_ratio(1, (stats.speed * 2) as u32);
            if chance {
                stats.speed += 1;
            }
        }
    }
    pub fn level_up_int(&mut self, subject_eid: &EntityID) {
        if let Some(stats) = self.components.stats.get_mut(subject_eid) {
            let chance = self.small_rng.gen_ratio(1, (stats.intelligence * 2) as u32);
            if chance {
                stats.intelligence += 1;
            }
        }
    }

    pub fn ranged_dodge(&mut self, subject_eid: &EntityID, object_eid: &EntityID) -> bool {
        let attacker_dodge = self.entity_dodge(subject_eid);

        let mut defender_dodge = self.entity_dodge(object_eid);
        if let Some(distance_between_ents) = self.distance_from_ent_to_ent(subject_eid, object_eid)
        {
            if let Some(equi) = self.components.equipments.get(subject_eid) {
                let wep_dist = equi.ranged_weapon.ideal_range();
                let min_dist = wep_dist / 3;
                let max_dist = wep_dist * 2;

                if (distance_between_ents < min_dist) || (distance_between_ents > max_dist) {
                    defender_dodge += distance_between_ents * 3;
                }
                defender_dodge += distance_between_ents;
                if attacker_dodge > defender_dodge {
                    self.level_up_int(subject_eid);
                    return true;
                }
            }
        }

        false
    }

    pub fn ranged_attack(&mut self, subject_eid: &EntityID, object_eid: &EntityID) -> ActionResult {
        let attacker_damage = self.attacker_ranged_damage(subject_eid);
        let defender_defense = self.entity_defense(object_eid);
        let defender_didnt_dodge = self.ranged_dodge(subject_eid, object_eid);

        let (enough_ammo, wep) = if let Some(equi) = self.components.equipments.get_mut(subject_eid)
        {
            match equi.ranged_weapon {
                RangedWeapon::Lųk => {
                    if equi.arrows > 0 {
                        equi.arrows -= 1;
                        (true, equi.ranged_weapon.clone())
                    } else {
                        (false, equi.ranged_weapon.clone())
                    }
                }
                RangedWeapon::Pråšča => {
                    if equi.bullets > 0 {
                        equi.bullets -= 1;
                        (true, equi.ranged_weapon.clone())
                    } else {
                        (false, equi.ranged_weapon.clone())
                    }
                }
                RangedWeapon::Oščěp => {
                    if equi.javelins > 0 {
                        equi.javelins -= 1;
                        (true, equi.ranged_weapon.clone())
                    } else {
                        (false, equi.ranged_weapon.clone())
                    }
                }
                RangedWeapon::Drotik => {
                    if equi.darts > 0 {
                        equi.darts -= 1;
                        (true, equi.ranged_weapon.clone())
                    } else {
                        (false, equi.ranged_weapon.clone())
                    }
                }
            }
        } else {
            (false, RangedWeapon::Lųk)
        };
        if enough_ammo {
            self.level_up_strength(subject_eid);
            self.level_up_speed(subject_eid);
            if let Some(defender_health) = self.components.healths.get_mut(object_eid) {
                if defender_didnt_dodge {
                    let real_damage = attacker_damage - defender_defense;
                    if real_damage > 0 {
                        defender_health.current_health -= real_damage;
                        return ActionResult::Success(
                            GameAction::RangedAttack(subject_eid.clone(), object_eid.clone()),
                            SuccessType::WithValueAndRangedWeapon(real_damage, wep),
                        );
                    } else {
                        return ActionResult::Failure(
                            GameAction::RangedAttack(subject_eid.clone(), object_eid.clone()),
                            FailType::Blocked,
                        );
                    }
                } else {
                    return ActionResult::Failure(
                        GameAction::RangedAttack(subject_eid.clone(), object_eid.clone()),
                        FailType::Miss,
                    );
                }
            }
        } else {
            return ActionResult::Failure(
                GameAction::RangedAttack(subject_eid.clone(), object_eid.clone()),
                FailType::NoAmmo,
            );
        }

        return ActionResult::Failure(
            GameAction::RangedAttack(subject_eid.clone(), object_eid.clone()),
            FailType::Normal,
        );
    }

    pub fn attacker_melee_damage(&mut self, subject_eid: &EntityID) -> i32 {
        let mut attacker_str = 0;
        let mut attacker_speed = 0;

        let mut attacker_damage = 0;
        if let Some(attacker_stats) = self.components.stats.get(subject_eid) {
            attacker_str = attacker_stats.strength.clone();
            attacker_speed = attacker_stats.speed.clone();
        }

        if let Some(equi) = self.components.equipments.get(subject_eid) {
            for itemik in &equi.equipped {
                if let EntityType::Item(ItemType::Weapon(wepik)) = self.get_ent_type(itemik) {
                    match wepik.damage_type() {
                        DamageType::Sharp => {
                            attacker_damage +=
                                self.small_rng.gen_range(0..=(attacker_speed + 2) / 2);
                        }
                        DamageType::Blunt => {
                            attacker_damage += self.small_rng.gen_range(0..=attacker_str + 1);
                        }
                    }
                    attacker_damage += self.small_rng.gen_range(0..=wepik.damage() + 1);
                }
            }
        }
        //attack with fists
        if attacker_damage == 0 {
            attacker_damage += self.small_rng.gen_range(0..=attacker_str + 1);
        }

        attacker_damage
    }
    pub fn attacker_ranged_damage(&mut self, subject_eid: &EntityID) -> i32 {
        let mut attacker_damage = 1;
        if let Some(attacker_stats) = self.components.stats.get(subject_eid) {
            attacker_damage += self
                .small_rng
                .gen_range(0..=(attacker_stats.strength / 4) + 1)
                + self.small_rng.gen_range(0..=(attacker_stats.speed / 3) + 1);
        }
        if let Some(equi) = self.components.equipments.get(subject_eid) {
            attacker_damage += self
                .small_rng
                .gen_range(0..=equi.ranged_weapon.damage() + 1);
        }

        attacker_damage
    }
    pub fn entity_dodge(&mut self, subject_eid: &EntityID) -> i32 {
        let mut attacker_dodge = self.small_rng.gen_range(0..7);
        if let Some(attacker_stats) = self.components.stats.get(subject_eid) {
            attacker_dodge += self.small_rng.gen_range(0..=attacker_stats.speed + 1)
                + (attacker_stats.intelligence / 3);
        }
        if let Some(equi) = self.components.equipments.get(subject_eid) {
            for itemik in &equi.equipped {
                if let EntityType::Item(ItemType::Weapon(wepik)) = self.get_ent_type(itemik) {
                    attacker_dodge += self.small_rng.gen_range(0..=wepik.weapon_length() + 1);
                }
            }
        }
        attacker_dodge
    }
    pub fn entity_defense(&mut self, subject_eid: &EntityID) -> i32 {
        let mut defender_defense = 1;
        if let Some(equi) = self.components.equipments.get(subject_eid) {
            for itemik in &equi.equipped {
                if let EntityType::Item(ItemType::Clothing(cloth)) = self.get_ent_type(itemik) {
                    defender_defense += self.small_rng.gen_range(0..=cloth.defense_value() + 1)
                }
            }
        }
        defender_defense
    }

    pub fn bump_attack(&mut self, subject_eid: &EntityID, object_eid: &EntityID) -> ActionResult {
        let attacker_damage = self.attacker_melee_damage(subject_eid);
        let defender_defense = self.entity_defense(object_eid);
        let attacker_dodge = self.entity_dodge(subject_eid);

        let defender_dodge = self.entity_dodge(object_eid);
        self.level_up_strength(subject_eid);
        self.level_up_speed(subject_eid);

        if attacker_dodge >= defender_dodge {
            self.level_up_speed(subject_eid);
            if attacker_damage > defender_defense {
                self.level_up_strength(subject_eid);

                self.level_up_int(subject_eid);
                let true_damage = attacker_damage - defender_defense;
                if let Some(defender_health) = self.components.healths.get_mut(object_eid) {
                    defender_health.current_health -= true_damage;
                    return ActionResult::Success(
                        GameAction::BumpAttack(subject_eid.clone(), object_eid.clone()),
                        SuccessType::WithValue(true_damage),
                    );
                } else {
                    return ActionResult::Failure(
                        GameAction::BumpAttack(subject_eid.clone(), object_eid.clone()),
                        FailType::Normal,
                    );
                }
            } else {
                return ActionResult::Failure(
                    GameAction::BumpAttack(subject_eid.clone(), object_eid.clone()),
                    FailType::Blocked,
                );
            }
        } else {
            return ActionResult::Failure(
                GameAction::BumpAttack(subject_eid.clone(), object_eid.clone()),
                FailType::Miss,
            );
        }
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
                    self.level_up_speed(subject_eid);
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

                    if let Some(EntityType::Item(ItemType::Ammo(ammik))) =
                        self.components.ent_types.get(item)
                    {
                        let amount = match ammik {
                            Ammo::Kulja(x) => {
                                self.components
                                    .equipments
                                    .get_mut(subject_eid)
                                    .unwrap()
                                    .bullets += x;
                                x
                            }
                            Ammo::Strěla(x) => {
                                self.components
                                    .equipments
                                    .get_mut(subject_eid)
                                    .unwrap()
                                    .arrows += x;
                                x
                            }
                            Ammo::Oščěp(x) => {
                                self.components
                                    .equipments
                                    .get_mut(subject_eid)
                                    .unwrap()
                                    .javelins += x;
                                x
                            }
                            Ammo::Drotik(x) => {
                                self.components
                                    .equipments
                                    .get_mut(subject_eid)
                                    .unwrap()
                                    .darts += x;
                                x
                            }
                        };
                        return ActionResult::Success(
                            GameAction::PickUp(subject_eid.clone(), item.clone()),
                            SuccessType::WithValue(amount.clone()),
                        );
                    } else {
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
        }
        ActionResult::Failure(
            GameAction::PickUp(subject_eid.clone(), item.clone()),
            FailType::Normal,
        )
    }
    pub fn consume_consumable(&mut self, subject_eid: &EntityID, item: Consumable) -> ActionResult {
        if let Some(hel) = self.components.healths.get_mut(subject_eid) {
            hel.current_health += item.health_effect();
            if hel.current_health > hel.max_health {
                hel.current_health = hel.max_health;
            }
        }
        if let Some(hel) = self.components.stats.get_mut(subject_eid) {
            hel.strength += item.strength_effect();
        }
        return ActionResult::Success(
            GameAction::Consume(subject_eid.clone(), item),
            SuccessType::Normal,
        );
    }

    pub fn equip_item_from_inv(&mut self, subject_eid: &EntityID, item: &EntityID) -> ActionResult {
        if let Some(boop) = self.components.equipments.get_mut(subject_eid) {
            if boop.inventory.contains(item) {
                let item_type = self.components.ent_types.get(item).unwrap().clone();

                if let EntityType::Item(itemik) = item_type {
                    match itemik {
                        ItemType::Weapon(wep) => {
                            let mut hand_space: i32 = 2;
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

                        ItemType::Ammo(amm) => {
                            //cant equip ammo it is used directly from inventory
                            return ActionResult::Failure(
                                GameAction::Equip(subject_eid.clone(), item.clone()),
                                FailType::WrongType,
                            );
                        }
                        ItemType::Consumable(consum) => {
                            boop.inventory.remove(item);
                            return self.consume_consumable(subject_eid, consum);
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

    pub fn generate_action_result_string(&self, act_resut: ActionResult) -> String {
        let line_text = match act_resut {
            ActionResult::Success(ga, reason) => match ga {
                GameAction::Consume(subj, consum) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_subj(&subj);

                    let verbik = ISV::l_participle("vypiti", &gender, &Number::Singular);

                    format!("{pronoun} {verbik} {}", consum)
                }
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
                            (ItemType::Weapon(_)) => "orųžati",
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
                GameAction::PickUp(subj, obj) => match reason {
                    SuccessType::WithValue(x) => {
                        let (pronoun, gender, person) = self.pronoun_for_act_subj(&subj);
                        let number = if x > 1 {
                            Number::Plural
                        } else {
                            Number::Singular
                        };
                        let object =
                            ISV::decline_noun(&self.get_entity_name(&obj), &Case::Acc, &number);
                        let verbik = ISV::conjugate_verb(
                            "podbirati",
                            &person,
                            &Number::Singular,
                            &gender,
                            &Tense::Present,
                        );

                        format!("{pronoun} {verbik} {} {}", x, object.0)
                    }
                    _ => {
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
                },
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
                                RangedWeapon::Oščěp => "kydati",
                                RangedWeapon::Drotik => "kydati",
                            };

                            let instik = match inst {
                                RangedWeapon::Lųk | RangedWeapon::Pråšča => {
                                    ISV::decline_noun(
                                        &format!("{}", inst),
                                        &Case::Ins,
                                        &Number::Singular,
                                    )
                                    .0
                                }

                                RangedWeapon::Oščěp | RangedWeapon::Drotik => {
                                    ISV::decline_noun(
                                        &format!("{}", inst),
                                        &Case::Acc,
                                        &Number::Singular,
                                    )
                                    .0
                                }
                            };

                            let verbik = ISV::conjugate_verb(
                                attack_verb,
                                &person,
                                &Number::Singular,
                                &gender,
                                &Tense::Present,
                            );
                            format!(
                                "{pronoun} {verbik} {instik} v {}, {nanositi} {val} točky škody",
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
                GameAction::Death(subj) => {
                    let name = self.get_entity_name(&subj);
                    format!("{name} umiraje")
                }
            },
            ActionResult::Failure(ga, reason) => match ga {
                GameAction::Consume(subj, consum) => {
                    let (pronoun, gender, person) = self.pronoun_for_act_subj(&subj);

                    format!("{pronoun} ne možeš vypiti  {}", consum)
                }
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

                GameAction::RangedAttack(subj, obj) => match reason {
                    FailType::Miss => {
                        if &subj == &self.local_player_id {
                            let dropped = self.pronoun_for_act_obj(&obj).0;

                            format!("ne udačno! ty propušćaješ ataku na {dropped}")
                        } else {
                            format!("")
                        }
                    }
                    FailType::Blocked => {
                        if &subj == &self.local_player_id {
                            let dropped = self.pronoun_for_act_subj(&obj).0;

                            format!("ne udačno! tvoju ataku zablokovali!")
                        } else {
                            format!("")
                        }
                    }
                    FailType::NoAmmo => String::from("ne imaješ dostatȯčno amunicije dlja ataky"),
                    _ => {
                        if &subj == &self.local_player_id {
                            let dropped = self.pronoun_for_act_obj(&obj).0;

                            format!("ty ne možeš atakovati {dropped}")
                        } else {
                            format!("")
                        }
                    }
                },
                GameAction::BumpAttack(subj, obj) => match reason {
                    FailType::Miss => {
                        if &subj == &self.local_player_id {
                            let dropped = self.pronoun_for_act_obj(&obj).0;

                            format!("ne udačno! ty propušćaješ ataku na {dropped}")
                        } else {
                            format!("")
                        }
                    }
                    FailType::Blocked => {
                        if &subj == &self.local_player_id {
                            let dropped = self.pronoun_for_act_subj(&obj).0;

                            format!("ne udačno! tvojų ataku zablokovali!")
                        } else {
                            format!("")
                        }
                    }
                    _ => {
                        if &subj == &self.local_player_id {
                            let dropped = self.pronoun_for_act_obj(&obj).0;

                            format!("ty ne možeš atakovati {dropped}")
                        } else {
                            format!("")
                        }
                    }
                },
                GameAction::Wait(subj) => {
                    format!("")
                }
                GameAction::Death(subj) => {
                    let name = self.get_entity_name(&subj);
                    format!("{name} umiraje")
                }
            },
        };

        line_text
    }
}
