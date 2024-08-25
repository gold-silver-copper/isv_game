use crate::*;
// ANCHOR: app
#[derive(Default)]
pub struct App {
    entity_counter: i64,
    components: ComponentHolder,
    input_state: InputState,

    inv_vecs: ItemVecs,

    exit: bool,
    game_map: GameMap,
    action_map: ActionMap,
    local_player_id: EntityID,
}
#[derive(Default)]
pub struct ItemVecs {
    selected_menu: ItemVecType,
    inventory: Vec<EntityID>,
    inv_list_state: ListState,
    equip_list_state: ListState,
    ground_list_state: ListState,
    inventory_names: Vec<String>,
    equipment: Vec<EntityID>,

    equipment_names: Vec<String>,
    ground: Vec<EntityID>,

    ground_names: Vec<String>,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        self.init();
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
            self.handle_actions()?;
            self.reload_ui();
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn init(&mut self) {
        let pik = (5, 5);

        self.local_player_id = self.spawn_player_at(&pik);
        self.spawn_item_at(&(5, 8), ItemType::Weapon(Weapon::Sword));
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        let lid = self.local_player_id.clone();

        match self.input_state {
            InputState::Basic => match key_event.code {
                KeyCode::Char(QUIT_BACK) => self.exit(),

                KeyCode::Char(CURSOR_UP) => {
                    self.action_map
                        .insert(lid, GameAction::Go(CardinalDirection::North));
                }
                KeyCode::Char(CURSOR_DOWN) => {
                    self.action_map
                        .insert(lid, GameAction::Go(CardinalDirection::South));
                }
                KeyCode::Char(CURSOR_LEFT) => {
                    self.action_map
                        .insert(lid, GameAction::Go(CardinalDirection::West));
                }
                KeyCode::Char(CURSOR_RIGHT) => {
                    self.action_map
                        .insert(lid, GameAction::Go(CardinalDirection::East));
                }
                KeyCode::Char(INVENTORY_MENU) => {
                    self.inv_vecs.ground_list_state.select_first();
                    self.input_state = InputState::Inventory;
                }

                _ => {}
            },
            InputState::Inventory => match key_event.code {
                KeyCode::Char(INVENTORY_MENU) => {
                    self.input_state = InputState::Basic;
                    self.inv_vecs.inv_list_state = ListState::default();
                    self.inv_vecs.ground_list_state = ListState::default();
                    self.inv_vecs.equip_list_state = ListState::default();
                    self.inv_vecs.selected_menu = ItemVecType::default();

                }
                KeyCode::Char(CURSOR_UP) => match self.inv_vecs.selected_menu {
                    ItemVecType::Inventory => self.inv_vecs.inv_list_state.select_previous(),
                    ItemVecType::Equipped => self.inv_vecs.equip_list_state.select_previous(),
                    ItemVecType::Ground => self.inv_vecs.ground_list_state.select_previous(),
                },
                KeyCode::Char(CURSOR_RIGHT) => match self.inv_vecs.selected_menu {
                    ItemVecType::Inventory => {
                        self.inv_vecs.selected_menu = ItemVecType::Equipped;
                        self.inv_vecs.inv_list_state = ListState::default();
                        self.inv_vecs.ground_list_state = ListState::default();
                        self.inv_vecs.equip_list_state.select_first();
                    }
                    ItemVecType::Equipped => {
                        self.inv_vecs.selected_menu = ItemVecType::Ground;
                        self.inv_vecs.inv_list_state = ListState::default();
                        self.inv_vecs.equip_list_state = ListState::default();
                        self.inv_vecs.ground_list_state.select_first();
                    }
                    ItemVecType::Ground => {
                        self.inv_vecs.selected_menu = ItemVecType::Inventory;
                        self.inv_vecs.ground_list_state = ListState::default();
                        self.inv_vecs.equip_list_state = ListState::default();
                        self.inv_vecs.inv_list_state.select_first();
                    }
                },

                KeyCode::Char(CURSOR_LEFT) => match self.inv_vecs.selected_menu {
                    ItemVecType::Inventory => {
                        self.inv_vecs.selected_menu = ItemVecType::Ground;
                        self.inv_vecs.inv_list_state = ListState::default();
                        self.inv_vecs.equip_list_state = ListState::default();
                        self.inv_vecs.ground_list_state.select_first();
                    }
                    ItemVecType::Equipped => {
                        self.inv_vecs.selected_menu = ItemVecType::Inventory;
                        self.inv_vecs.ground_list_state = ListState::default();
                        self.inv_vecs.equip_list_state = ListState::default();
                        self.inv_vecs.inv_list_state.select_first();
                    }
                    ItemVecType::Ground => {
                        self.inv_vecs.selected_menu = ItemVecType::Equipped;
                        self.inv_vecs.inv_list_state = ListState::default();
                        self.inv_vecs.ground_list_state = ListState::default();
                        self.inv_vecs.equip_list_state.select_first();
                    }
                },

                KeyCode::Char(CURSOR_DOWN) => match self.inv_vecs.selected_menu {
                    ItemVecType::Inventory => {
                        if let Some(invlen) = self.inv_vecs.inv_list_state.selected() {
                            if invlen < self.inv_vecs.inventory.len() - 1 {
                                self.inv_vecs.inv_list_state.select_next();
                            }
                        }
                    }
                    ItemVecType::Equipped => {
                        if let Some(invlen) = self.inv_vecs.equip_list_state.selected() {
                            if invlen < self.inv_vecs.equipment.len() - 1 {
                                self.inv_vecs.equip_list_state.select_next();
                            }
                        }
                    }
                    ItemVecType::Ground => {
                        if let Some(invlen) = self.inv_vecs.ground_list_state.selected() {
                            if invlen < self.inv_vecs.ground.len() - 1 {
                                self.inv_vecs.ground_list_state.select_next();
                            }
                        }
                    }
                },
                KeyCode::Char(DROP_UNEQUIP_ACTION) => match self.inv_vecs.selected_menu {
                    ItemVecType::Equipped => {
                        let (possible, selected_id) =
                            self.manage_item_vec_input(&self.inv_vecs.selected_menu);
                        if possible {
                            self.action_map
                                .insert(lid, GameAction::UnEquip(selected_id));
                        }
                    }
                    ItemVecType::Inventory => {
                        let (possible, selected_id) =
                            self.manage_item_vec_input(&self.inv_vecs.selected_menu);
                        if possible {
                            self.action_map.insert(lid, GameAction::Drop(selected_id));
                        }
                    }
                    _ => (),
                },
                KeyCode::Char(PICKUP_EQUIP_ACTION) => match self.inv_vecs.selected_menu {
                    ItemVecType::Ground => {
                        let (possible, selected_id) =
                            self.manage_item_vec_input(&self.inv_vecs.selected_menu);
                        if possible {
                            self.action_map.insert(lid, GameAction::PickUp(selected_id));
                        }
                    }
                    ItemVecType::Inventory => {
                        let (possible, selected_id) =
                            self.manage_item_vec_input(&self.inv_vecs.selected_menu);
                        if possible {
                            self.action_map.insert(lid, GameAction::Equip(selected_id));
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

    fn manage_item_vec_input(&self, itemvectype: &ItemVecType) -> (bool, EntityID) {
        let boop = match itemvectype {
            ItemVecType::Equipped => self.inv_vecs.equip_list_state.selected(),
            ItemVecType::Inventory => self.inv_vecs.inv_list_state.selected(),
            ItemVecType::Ground => self.inv_vecs.ground_list_state.selected(),
        };

        if let Some(sid) = boop {
            let moop = match itemvectype {
                ItemVecType::Equipped => self.inv_vecs.equipment.get(sid),
                ItemVecType::Inventory => self.inv_vecs.inventory.get(sid),
                ItemVecType::Ground => self.inv_vecs.ground.get(sid),
            };

            if let Some(id_to_pickup) = moop {
                let id_to_pickup = id_to_pickup.clone();
                let lid = self.local_player_id.clone();

                return (true, id_to_pickup);
            }
        }

        (false, 0)
    }

    fn handle_movement(&mut self, eid: &EntityID, cd: &CardinalDirection) -> Result<()> {
        let xyik = cd.to_xyz();
        if let Some(e_pos) = self.components.positions.get_mut(eid) {
            let destination = (e_pos.0 + xyik.0, e_pos.1 + xyik.1);
            // println!("epos got");

            if let Some(dest_vox) = self.game_map.get_mut_voxel_at(&destination) {
                if !dest_vox.blocks_movement(&self.components.ent_types) {
                    // println!("dest no block");
                    dest_vox.entity_set.insert(eid.clone());

                    if let Some(origin_vox) = self.game_map.get_mut_voxel_at(e_pos) {
                        origin_vox.entity_set.remove(eid);
                    }
                    *e_pos = destination;
                }
            }
        }

        Ok(())
    }
    fn reload_ui(&mut self) {
        match self.input_state {
            InputState::Inventory => {
                self.generate_inventory_eid_vec();
                self.generate_equipped_eid_vec();
                self.generate_ground_item_eid_vec();

                if let Some(sel_len) = self.inv_vecs.equip_list_state.selected_mut() {
                    if *sel_len > self.inv_vecs.equipment.len() - 1 {
                        *sel_len = self.inv_vecs.equipment.len() - 1;
                    }
                }
                if let Some(sel_len) = self.inv_vecs.inv_list_state.selected_mut() {
                    if *sel_len > self.inv_vecs.inventory.len() - 1 {
                        *sel_len = self.inv_vecs.inventory.len() - 1;
                    }
                }
                if let Some(sel_len) = self.inv_vecs.ground_list_state.selected_mut() {
                    if *sel_len > self.inv_vecs.ground.len() - 1 {
                        *sel_len = self.inv_vecs.ground.len() - 1;
                    }
                }
            }

            _ => (),
        }
    }

    fn handle_actions(&mut self) -> Result<()> {
        let a_map = self.action_map.clone();
        self.action_map.drain();

        for (eid, act) in a_map {
            //println!("moving");

            match act {
                GameAction::Go(cd) => self.handle_movement(&eid, &cd)?,
                GameAction::Drop(iid) => self.drop_item_from_inv(&iid, &eid),
                GameAction::PickUp(iid) => self.pickup_item_from_ground(&iid, &eid),
                GameAction::Equip(iid) => self.equip_item_from_inv(&iid, &eid),
                GameAction::UnEquip(iid) => self.unequip_item_from_equipped(&iid, &eid),
                _ => panic!("meow"),
            }
        }

        Ok(())
    }

    pub fn spawn_player_at(&mut self, point: &MyPoint) -> EntityID {
        let pid = self.spawn_animal_at(point);
        let iid = self.create_item(ItemType::Weapon(Weapon::Sword));
        let iid2 = self.create_item(ItemType::Clothing(Clothing::Toga));
        let iid3 = self.create_item(ItemType::Weapon(Weapon::Mace));

        let player_equip = self
            .components
            .equipments
            .get_mut(&pid)
            .expect("MUST HAVE QUEIP");
        player_equip.equipped.insert(iid);
        player_equip.inventory.insert(iid2);
        player_equip.inventory.insert(iid3);

        pid
    }

    pub fn spawn_animal_at(&mut self, point: &MyPoint) -> EntityID {
        let eid = self.get_unique_eid();
        self.components.positions.insert(eid.clone(), point.clone());
        self.components
            .ent_types
            .insert(eid.clone(), EntityType::Animalia);
        self.components
            .equipments
            .insert(eid.clone(), Equipment::default());
        self.components.healths.insert(eid.clone(), Health::default());

        let voxik = self
            .game_map
            .get_mut_voxel_at(point)
            .expect("cant spawn ent in empty voxel");

        voxik.entity_set.insert(eid.clone());

        eid.clone()
    }

    pub fn create_item(&mut self, item: ItemType) -> EntityID {
        let eid = self.get_unique_eid();

        self.components
            .ent_types
            .insert(eid.clone(), EntityType::Item(item));

        eid
    }

    pub fn drop_item_from_inv(&mut self, item: &EntityID, holder: &EntityID) {
        let holder_inv = self.components.equipments.get_mut(holder).unwrap();
        let holder_pos = self.components.positions.get(holder).unwrap();

        if holder_inv.inventory.contains(item) {
            holder_inv.inventory.remove(item);
            let holder_vox = self.game_map.get_mut_voxel_at(holder_pos).unwrap();
            holder_vox.entity_set.insert(item.clone());
        }
    }

    pub fn set_ent_position(&mut self, eid: &EntityID, point: &MyPoint) {
        self.components.positions.insert(eid.clone(), point.clone());

        let voxik = self
            .game_map
            .get_mut_voxel_at(point)
            .expect("cant spawn ent in empty voxel");

        voxik.entity_set.insert(eid.clone());
    }

    pub fn spawn_item_at(&mut self, point: &MyPoint, item: ItemType) -> EntityID {
        let eid = self.create_item(item);
        self.set_ent_position(&eid, point);

        eid
    }

    pub fn generate_info_paragraph(&self) -> Paragraph {
        let player_equip = self
            .components
            .equipments
            .get(&self.local_player_id)
            .expect("PLAYER MUST HAVE EQUIPMENT COMPONENT");

        let mut wield_string = String::new();

        if player_equip.equipped.is_empty() {
            wield_string = String::from("nothing")
        } else {
            for (item) in player_equip.equipped.iter() {
                let item_type = self
                    .components
                    .ent_types
                    .get(item)
                    .expect("EVERY ITEM MUST HAVE AN ENTITY TYPE");
                let item_name = match item_type {
                    EntityType::Animalia => panic!("CANNOT WIELD ANIMALS... yet"),
                    EntityType::Item(itemik) => itemik.item_name(),
                };
                wield_string.push_str(&format!(" {item_name}"));
            }
        }

        let mut lines = (Text::from(vec![
            Line::from("HAI"),
            Line::from("Wielding..."),
            Line::from(wield_string),
        ]));

        Paragraph::new(Text::from(lines))
            .on_black()
            .block(Block::bordered())
    }

    pub fn render_item_list(&self, title: &str, itemvectype: ItemVecType) -> List {
        let wut = match itemvectype {
            ItemVecType::Equipped => self.inv_vecs.equipment_names.clone(),
            ItemVecType::Inventory => self.inv_vecs.inventory_names.clone(),
            ItemVecType::Ground => self.inv_vecs.ground_names.clone(),
        };

        let list = List::new(wut)
            .block(Block::bordered().title(title.to_string()))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">")
            .repeat_highlight_symbol(true);

        list
    }

    pub fn gen_item_name_vec(&mut self, id_vec: &Vec<EntityID>) -> Vec<String> {
        let mut itemnamevec = Vec::new();

        for itik in id_vec.iter() {
            let typik = self
                .components
                .ent_types
                .get(itik)
                .expect("ent type must have");
            let itname = match typik {
                EntityType::Animalia => {
                    panic!("why do u have an animal in ur inventory")
                }
                EntityType::Item(itemik) => itemik.item_name(),
            };
            itemnamevec.push(itname);
        }
        itemnamevec
    }
    pub fn generate_inventory_eid_vec(&mut self) {
        let mut evec = Vec::new();
        let player_inv = self
            .components
            .equipments
            .get(&self.local_player_id)
            .expect("must have equi");

        for itemik in player_inv.inventory.iter() {
            evec.push(itemik.clone());
        }

        self.inv_vecs.inventory_names = self.gen_item_name_vec(&evec);

        self.inv_vecs.inventory = evec;
    }
    pub fn generate_equipped_eid_vec(&mut self) {
        let mut evec = Vec::new();
        let player_equi = self
            .components
            .equipments
            .get(&self.local_player_id)
            .expect("must have equi");

        for itemik in player_equi.equipped.iter() {
            evec.push(itemik.clone());
        }

        self.inv_vecs.equipment_names = self.gen_item_name_vec(&evec);

        self.inv_vecs.equipment = evec;
    }

    pub fn generate_ground_item_eid_vec(&mut self) {
        let mut evec = Vec::new();
        let player_pos = self
            .components
            .positions
            .get(&self.local_player_id)
            .unwrap();
        let player_vox = self.game_map.get_voxel_at(player_pos).unwrap();

        for boop in player_vox.entity_set.iter() {
            let booptype = self.components.ent_types.get(boop).unwrap();
            match booptype {
                EntityType::Animalia => {}
                EntityType::Item(x) => {
                    evec.push(boop.clone());
                }
            }
        }

        self.inv_vecs.ground_names = self.gen_item_name_vec(&evec);

        self.inv_vecs.ground = evec;
    }

    pub fn pickup_item_from_ground(&mut self, item: &EntityID, pickerupper: &EntityID) {
        if let Some(ent_pos) = self.components.positions.get(pickerupper) {
            if let Some(ent_vox) = self.game_map.get_mut_voxel_at(ent_pos) {
                if ent_vox.entity_set.contains(item) {
                    ent_vox.entity_set.remove(item);
                    self.components
                        .equipments
                        .get_mut(pickerupper)
                        .unwrap()
                        .inventory
                        .insert(item.clone());
                }
            }
        }
    }

    pub fn equip_item_from_inv(&mut self, item: &EntityID, equipper: &EntityID) {
        if let Some(boop) = self.components.equipments.get_mut(equipper) {
            if boop.inventory.contains(item) {
                boop.inventory.remove(item);
                boop.equipped.insert(item.clone());
            }
        }
    }
    pub fn unequip_item_from_equipped(&mut self, item: &EntityID, equipper: &EntityID) {
        if let Some(boop) = self.components.equipments.get_mut(equipper) {
            if boop.equipped.contains(item) {
                boop.equipped.remove(item);
                boop.inventory.insert(item.clone());
            }
        }
    }

    pub fn get_unique_eid(&mut self) -> EntityID {
        self.entity_counter += 1;
        self.entity_counter.clone()
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(80), Constraint::Min(20)],
        )
        .split(area);

        let client_pos = self
            .components
            .positions
            .get(&self.local_player_id)
            .unwrap_or(&(0, 0));

        let client_render = self.game_map.create_client_render_packet_for_entity(
            &client_pos,
            &layout[0],
            &self.components.ent_types,
        );

        let client_graphics = client_render.voxel_grid;
        let client_visible_ents = client_render.ent_vec;
        //    ui_resources.visible_ents = client_visible_ents;

        let mut render_lines = Vec::new();
        let needed_height = layout[0].height as i16;

        if client_graphics.len() > 0 {
            for y in (0..needed_height) {
                let myspanvec: Vec<_> = client_graphics[y as usize]
                    .iter()
                    .map(|x| Span::from(x.0).fg(x.1).bg(x.2))
                    .collect();

                let myline = ratatui::text::Line::from(myspanvec);

                render_lines.push(myline);
            }
        }

        let mut inv_state = self.inv_vecs.inv_list_state.clone();
        let mut ground_state = self.inv_vecs.ground_list_state.clone();
        let mut equip_state = self.inv_vecs.equip_list_state.clone();

        //neccesary beccause drawing is from the top
        render_lines.reverse();
        Paragraph::new(Text::from(render_lines))
            .on_black()
            .block(Block::new())
            .render(layout[0], buf);
        self.generate_info_paragraph().render(layout[1], buf);

        match self.input_state {
            InputState::Basic => (),

            InputState::Inventory => {
                let block = Block::bordered().title("Popup");
                let pop_area = popup_area(layout[0], 80, 70);
                let pop_layout = Layout::new(
                    Direction::Horizontal,
                    [
                        Constraint::Min(20),
                        Constraint::Min(20),
                        Constraint::Min(20),
                    ],
                )
                .split(pop_area);
                Clear.render(pop_area, buf); //this clears out the background
                block.render(pop_area, buf); //this clears out the background
                ratatui::prelude::StatefulWidget::render(
                    self.render_item_list("Inventory", ItemVecType::Inventory),
                    pop_layout[1],
                    buf,
                    &mut inv_state,
                );
                ratatui::prelude::StatefulWidget::render(
                    self.render_item_list("Equipped", ItemVecType::Equipped),
                    pop_layout[2],
                    buf,
                    &mut equip_state,
                );
                ratatui::prelude::StatefulWidget::render(
                    self.render_item_list("Ground", ItemVecType::Ground),
                    pop_layout[0],
                    buf,
                    &mut ground_state,
                );
            }

            _ => panic!("INPUT STATE RENDER NOT IMPELEMNTED"),
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
