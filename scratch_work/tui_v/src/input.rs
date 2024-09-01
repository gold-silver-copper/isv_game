use crate::*;

impl App {
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        let lid = self.local_player_id.clone();

        match self.input_state {
            InputState::Basic => match key_event.code {
                KeyCode::Char(QUIT_BACK) => self.exit(),

                KeyCode::Char(CURSOR_UP) => {
                    self.action_vec
                        .push(GameAction::Go(lid, CardinalDirection::North));
                }
                KeyCode::Char(CURSOR_UP_LEFT) => {
                    self.action_vec
                        .push(GameAction::Go(lid, CardinalDirection::NorthWest));
                }
                KeyCode::Char(CURSOR_UP_RIGHT) => {
                    self.action_vec
                        .push(GameAction::Go(lid, CardinalDirection::NorthEast));
                }
                KeyCode::Char(CURSOR_DOWN_LEFT) => {
                    self.action_vec
                        .push(GameAction::Go(lid, CardinalDirection::SouthWest));
                }
                KeyCode::Char(CURSOR_DOWN_RIGHT) => {
                    self.action_vec
                        .push(GameAction::Go(lid, CardinalDirection::SouthEast));
                }
                KeyCode::Char(CURSOR_DOWN) => {
                    self.action_vec
                        .push(GameAction::Go(lid, CardinalDirection::South));
                }
                KeyCode::Char(CURSOR_LEFT) => {
                    self.action_vec
                        .push(GameAction::Go(lid, CardinalDirection::West));
                }
                KeyCode::Char(CURSOR_RIGHT) => {
                    self.action_vec
                        .push(GameAction::Go(lid, CardinalDirection::East));
                }
                KeyCode::Char(INVENTORY_MENU) => {
                    self.inv_vecs.item_list_state.select_first();
                    self.input_state = InputState::Inventory;
                }
                KeyCode::Char(WAIT_KEY) => {
                    self.action_vec.push(GameAction::Wait(lid));
                }

                _ => {}
            },
            InputState::Inventory => match key_event.code {
                KeyCode::Char(INVENTORY_MENU) => {
                    self.input_state = InputState::Basic;
                    self.inv_vecs.item_list_state = ListState::default();

                    self.inv_vecs.selected_menu = ItemVecType::default();
                }
                KeyCode::Char(CURSOR_UP) => self.inv_vecs.item_list_state.select_previous(),
                KeyCode::Char(CURSOR_RIGHT) => match self.inv_vecs.selected_menu {
                    ItemVecType::Inventory => {
                        self.inv_vecs.selected_menu = ItemVecType::Equipped;

                        self.inv_vecs.item_list_state.select_first();
                    }
                    ItemVecType::Equipped => {
                        self.inv_vecs.selected_menu = ItemVecType::Ground;

                        self.inv_vecs.item_list_state.select_first();
                    }
                    ItemVecType::Ground => {
                        self.inv_vecs.selected_menu = ItemVecType::Inventory;

                        self.inv_vecs.item_list_state.select_first();
                    }
                },

                KeyCode::Char(CURSOR_LEFT) => match self.inv_vecs.selected_menu {
                    ItemVecType::Inventory => {
                        self.inv_vecs.selected_menu = ItemVecType::Ground;

                        self.inv_vecs.item_list_state.select_first();
                    }
                    ItemVecType::Equipped => {
                        self.inv_vecs.selected_menu = ItemVecType::Inventory;

                        self.inv_vecs.item_list_state.select_first();
                    }
                    ItemVecType::Ground => {
                        self.inv_vecs.selected_menu = ItemVecType::Equipped;

                        self.inv_vecs.item_list_state.select_first();
                    }
                },

                KeyCode::Char(CURSOR_DOWN) => match self.inv_vecs.selected_menu {
                    ItemVecType::Inventory => {
                        if let Some(invlen) = self.inv_vecs.item_list_state.selected() {
                            if invlen < self.inv_vecs.inventory.len() - 1 {
                                self.inv_vecs.item_list_state.select_next();
                            }
                        }
                    }
                    ItemVecType::Equipped => {
                        if let Some(invlen) = self.inv_vecs.item_list_state.selected() {
                            if invlen < self.inv_vecs.equipment.len() - 1 {
                                self.inv_vecs.item_list_state.select_next();
                            }
                        }
                    }
                    ItemVecType::Ground => {
                        if let Some(invlen) = self.inv_vecs.item_list_state.selected() {
                            if invlen < self.inv_vecs.ground.len() - 1 {
                                self.inv_vecs.item_list_state.select_next();
                            }
                        }
                    }
                },
                KeyCode::Char(DROP_UNEQUIP_ACTION) => match self.inv_vecs.selected_menu {
                    ItemVecType::Equipped => {
                        let (possible, selected_id) =
                            self.manage_item_vec_input(&self.inv_vecs.selected_menu);
                        if possible {
                            self.action_vec.push(GameAction::UnEquip(lid, selected_id));
                        }
                    }
                    ItemVecType::Inventory => {
                        let (possible, selected_id) =
                            self.manage_item_vec_input(&self.inv_vecs.selected_menu);
                        if possible {
                            self.action_vec.push(GameAction::Drop(lid, selected_id));
                        }
                    }
                    _ => (),
                },
                KeyCode::Char(PICKUP_EQUIP_ACTION) => match self.inv_vecs.selected_menu {
                    ItemVecType::Ground => {
                        let (possible, selected_id) =
                            self.manage_item_vec_input(&self.inv_vecs.selected_menu);
                        if possible {
                            self.action_vec.push(GameAction::PickUp(lid, selected_id));
                        }
                    }
                    ItemVecType::Inventory => {
                        let (possible, selected_id) =
                            self.manage_item_vec_input(&self.inv_vecs.selected_menu);
                        if possible {
                            self.action_vec.push(GameAction::Equip(lid, selected_id));
                        }
                    }
                    _ => (),
                },

                _ => {}
            },

            _ => panic!("input state not implemented"),
        }

        Ok(())
    }
}
